import { Telegraf, Context } from 'telegraf';
import { Update, Message } from 'telegraf/typings/core/types/typegram';
import * as fheActions from '../actions/fheActions';

type TelegramConfig = {
  botToken: string;
  allowDirectMessages?: boolean;
  shouldOnlyJoinInAllowedGroups?: boolean;
  allowedGroupIds?: string[];
  messageTrackingLimit?: number;
  fheServerUrl?: string;
};

export class TelegramService {
  private bot: Telegraf;
  private config: TelegramConfig;
  private fheConfig: { serverUrl: string };
  private messageHistory: Map<number, Message[]> = new Map();

  constructor(config: TelegramConfig) {
    if (!config.botToken) {
      throw new Error('Telegram bot token is required');
    }

    this.config = {
      allowDirectMessages: config.allowDirectMessages ?? false,
      shouldOnlyJoinInAllowedGroups: config.shouldOnlyJoinInAllowedGroups ?? false,
      allowedGroupIds: config.allowedGroupIds ?? [],
      messageTrackingLimit: config.messageTrackingLimit ?? 100,
      ...config
    };

    this.fheConfig = {
      serverUrl: config.fheServerUrl || 'http://localhost:3000'
    };
    
    this.bot = new Telegraf(config.botToken);
    this.setupHandlers();
  }

  private setupHandlers() {
    // Handle start command
    this.bot.start((ctx) => {
      return ctx.reply(
        'Hello! I am your SVM-FHE assistant. I can help you manage encrypted values on the Solana blockchain.\n\n' +
        'Available commands:\n' +
        '/deposit <value> - Deposit a value to be encrypted\n' +
        '/balance - Check your encrypted balance\n' +
        '/transfer <recipient_id> <amount> - Transfer encrypted funds\n' +
        '/help - Show this help message'
      );
    });

    // Handle help command
    this.bot.help((ctx) => {
      return ctx.reply(
        'SVM-FHE Bot Commands:\n\n' +
        '/deposit <value> - Deposit a value to be encrypted\n' +
        '/balance - Check your encrypted balance\n' +
        '/transfer <recipient_id> <amount> - Transfer encrypted funds\n' +
        '/status - Check if the FHE server is running\n' +
        '/help - Show this help message'
      );
    });

    // Handle deposit command
    this.bot.command('deposit', async (ctx) => {
      try {
        const args = ctx.message.text.split(' ');
        if (args.length !== 2) {
          return ctx.reply('Usage: /deposit <value>');
        }

        const value = parseInt(args[1]);
        if (isNaN(value) || value <= 0) {
          return ctx.reply('Please provide a valid positive number');
        }

        // Generate a key based on the user's Telegram ID
        const { key } = fheActions.generateKeyFromUserId(ctx.message.from.id.toString());
        
        // Make deposit request to FHE server
        const result = await fheActions.deposit(key, value, this.fheConfig);

        if (result.success) {
          return ctx.reply(`Successfully deposited ${value} tokens into your encrypted account.`);
        } else {
          return ctx.reply('There was an error processing your deposit.');
        }
      } catch (error) {
        console.error('Deposit error:', error);
        return ctx.reply('Sorry, there was an error processing your request.');
      }
    });

    // Handle balance command
    this.bot.command('balance', async (ctx) => {
      try {
        // Generate a key based on the user's Telegram ID
        const { key } = fheActions.generateKeyFromUserId(ctx.message.from.id.toString());
        
        // Make decrypt request to FHE server
        const result = await fheActions.decrypt(key, this.fheConfig);

        if (result.success) {
          return ctx.reply(`Your encrypted balance is: ${result.value} tokens`);
        } else {
          return ctx.reply('Could not retrieve your balance. You may need to make a deposit first.');
        }
      } catch (error) {
        console.error('Balance check error:', error);
        return ctx.reply('Sorry, there was an error retrieving your balance.');
      }
    });

    // Handle transfer command
    this.bot.command('transfer', async (ctx) => {
      try {
        const args = ctx.message.text.split(' ');
        if (args.length !== 3) {
          return ctx.reply('Usage: /transfer <recipient_id> <amount>');
        }

        const recipientId = parseInt(args[1]);
        const amount = parseInt(args[2]);

        if (isNaN(recipientId)) {
          return ctx.reply('Please provide a valid recipient ID');
        }

        if (isNaN(amount) || amount <= 0) {
          return ctx.reply('Please provide a valid positive amount');
        }

        // Generate keys based on user IDs
        const { key: senderKey } = fheActions.generateKeyFromUserId(ctx.message.from.id.toString());
        const { key: recipientKey } = fheActions.generateKeyFromUserId(recipientId.toString());
        
        // First, deposit the transfer amount to get an encrypted value
        await fheActions.deposit([0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], amount, this.fheConfig);

        // Now make transfer request to FHE server
        const result = await fheActions.transfer(
          senderKey,
          recipientKey,
          [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
          this.fheConfig
        );

        if (result.success) {
          return ctx.reply(`Successfully transferred ${amount} tokens to user ID ${recipientId}`);
        } else {
          return ctx.reply('There was an error processing your transfer. Make sure you have sufficient funds.');
        }
      } catch (error) {
        console.error('Transfer error:', error);
        return ctx.reply('Sorry, there was an error processing your transfer request.');
      }
    });

    // Handle status command
    this.bot.command('status', async (ctx) => {
      try {
        // Check if FHE server is running
        const result = await fheActions.checkServerStatus(this.fheConfig);

        if (result.status === 'online') {
          return ctx.reply('✅ The FHE server is online and operational.');
        } else {
          return ctx.reply('❌ The FHE server appears to be offline or not responding.');
        }
      } catch (error) {
        console.error('Status check error:', error);
        return ctx.reply('❌ The FHE server appears to be offline or not responding.');
      }
    });

    // Handle messages in groups
    this.bot.on('message', (ctx) => {
      const message = ctx.message;
      if ('chat' in message && message.chat.type !== 'private') {
        this.handleGroupMessage(ctx);
      }
    });
  }

  private handleGroupMessage(ctx: Context<Update>) {
    // Check if we should only join allowed groups
    if (this.config.shouldOnlyJoinInAllowedGroups && 'chat' in ctx.message) {
      const chatId = ctx.message.chat.id.toString();
      if (!this.config.allowedGroupIds?.includes(chatId)) {
        console.log(`Ignoring message from non-allowed group: ${chatId}`);
        return;
      }
    }

    // Track message history
    if ('chat' in ctx.message) {
      const chatId = ctx.message.chat.id;
      if (!this.messageHistory.has(chatId)) {
        this.messageHistory.set(chatId, []);
      }
      
      const history = this.messageHistory.get(chatId)!;
      history.push(ctx.message);
      
      // Limit history size
      if (history.length > this.config.messageTrackingLimit!) {
        history.shift();
      }
    }
  }

  public async start() {
    try {
      await this.bot.launch();
      console.log('Telegram bot started successfully');
      return true;
    } catch (error) {
      console.error('Failed to start Telegram bot:', error);
      return false;
    }
  }

  public async stop() {
    try {
      this.bot.stop();
      console.log('Telegram bot stopped successfully');
      return true;
    } catch (error) {
      console.error('Failed to stop Telegram bot:', error);
      return false;
    }
  }
}
