import type { Plugin, ActionContext } from '@elizaos/core';
import * as fheActions from './actions/fheActions';
import { TelegramService } from './services/telegramService';

/**
 * SVM-FHE Plugin for ElizaOS
 * 
 * This plugin enables ElizaOS to interact with the Solana FHE server for
 * performing encrypted operations on the Solana blockchain.
 */
export default class SvmFhePlugin implements Plugin {
  name = 'svm-fhe';
  private fheServerUrl: string;
  private telegramService: TelegramService | null = null;

  constructor(config: { 
    fheServerUrl?: string, 
    telegram?: {
      botToken: string;
      allowDirectMessages?: boolean;
      shouldOnlyJoinInAllowedGroups?: boolean;
      allowedGroupIds?: string[];
      messageTrackingLimit?: number;
    }
  } = {}) {
    this.fheServerUrl = config.fheServerUrl || 'http://localhost:3000';

    // Initialize Telegram service if config is provided
    if (config.telegram?.botToken) {
      this.telegramService = new TelegramService({
        ...config.telegram,
        fheServerUrl: this.fheServerUrl
      });
    }
  }

  async initialize(): Promise<void> {
    // Check if FHE server is available
    try {
      // Try to access the zero key as a basic connectivity test
      const result = await fheActions.checkServerStatus({ serverUrl: this.fheServerUrl });
      if (result.status === 'online') {
        console.log('SVM-FHE Plugin: Successfully connected to FHE server');
      } else {
        console.error('SVM-FHE Plugin: Failed to connect to FHE server');
      }
      
      // Start Telegram service if available
      if (this.telegramService) {
        const started = await this.telegramService.start();
        if (started) {
          console.log('SVM-FHE Plugin: Telegram service started successfully');
        } else {
          console.error('SVM-FHE Plugin: Failed to start Telegram service');
        }
      }
    } catch (error) {
      console.error('SVM-FHE Plugin: Failed to connect to FHE server', error);
    }
  }

  async cleanup(): Promise<void> {
    // Stop Telegram service if running
    if (this.telegramService) {
      await this.telegramService.stop();
      console.log('SVM-FHE Plugin: Telegram service stopped');
    }
  }

  getActions() {
    return {
      /**
       * Deposit a value to be encrypted
       * @param key - 32-byte array representing the account key
       * @param value - The value to encrypt and store
       */
      deposit: async (ctx: ActionContext, key: number[], value: number) => {
        return await fheActions.deposit(key, value, { serverUrl: this.fheServerUrl });
      },

      /**
       * Decrypt and retrieve a value
       * @param key - 32-byte array representing the account key
       */
      decrypt: async (ctx: ActionContext, key: number[]) => {
        return await fheActions.decrypt(key, { serverUrl: this.fheServerUrl });
      },

      /**
       * Transfer encrypted value between accounts
       * @param senderKey - 32-byte array representing the sender's account key
       * @param recipientKey - 32-byte array representing the recipient's account key
       * @param transferValueKey - 32-byte array representing the transfer amount key
       */
      transfer: async (ctx: ActionContext, senderKey: number[], recipientKey: number[], transferValueKey: number[]) => {
        return await fheActions.transfer(senderKey, recipientKey, transferValueKey, { serverUrl: this.fheServerUrl });
      },

      /**
       * Withdraw encrypted value
       * @param key - 32-byte array representing the account key
       * @param valueKey - 32-byte array representing the withdrawal amount key
       */
      withdraw: async (ctx: ActionContext, key: number[], valueKey: number[]) => {
        return await fheActions.withdraw(key, valueKey, { serverUrl: this.fheServerUrl });
      },

      /**
       * Generate a random 32-byte key array
       */
      generateKey: async (ctx: ActionContext) => {
        return fheActions.generateKey();
      },

      /**
       * Generate a key from a user ID
       */
      generateKeyFromUserId: async (ctx: ActionContext, userId: string) => {
        return fheActions.generateKeyFromUserId(userId);
      },

      /**
       * Check if the FHE server is running
       */
      checkServerStatus: async (ctx: ActionContext) => {
        return await fheActions.checkServerStatus({ serverUrl: this.fheServerUrl });
      },

      /**
       * Start the Telegram bot service
       * @param botToken - Telegram bot token
       */
      startTelegramBot: async (ctx: ActionContext, botToken: string, options: {
        allowDirectMessages?: boolean;
        shouldOnlyJoinInAllowedGroups?: boolean;
        allowedGroupIds?: string[];
      } = {}) => {
        try {
          if (!this.telegramService) {
            this.telegramService = new TelegramService({
              botToken,
              ...options,
              fheServerUrl: this.fheServerUrl
            });
          }
          
          const started = await this.telegramService.start();
          return { success: started };
        } catch (error) {
          console.error('Failed to start Telegram bot:', error);
          return { success: false, error: (error as Error).message };
        }
      },

      /**
       * Stop the Telegram bot service
       */
      stopTelegramBot: async (ctx: ActionContext) => {
        try {
          if (this.telegramService) {
            const stopped = await this.telegramService.stop();
            return { success: stopped };
          }
          return { success: false, error: 'Telegram bot not running' };
        } catch (error) {
          console.error('Failed to stop Telegram bot:', error);
          return { success: false, error: (error as Error).message };
        }
      }
    };
  }
}
