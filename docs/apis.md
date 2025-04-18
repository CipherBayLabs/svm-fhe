These are the APIs that applications and relayers can use to interact with the FHE Server Backend.

## Post
    - This is the primary endpoint for encrypting u64 values.
    - **Endpoint**: `POST /post`
    - **Description**: Encrypts a plaintext value and stores it with the provided key
    - **Request Body**:
    ```json
    {
      "value": 1000,          // Plaintext u64 value to encrypt
      "key": [u8; 32]         // 32-byte array key to identify the stored value
    }
    ```
    - **Response**: 200 OK on success

## Transfer
    - **Endpoint**: `POST /transfer`
    - **Description**: Transfers encrypted value between accounts
    - **Request Body**:
    ```json
    {
      "sender_key": [u8; 32],      // 32-byte array of sender's key
      "recipient_key": [u8; 32],    // 32-byte array of recipient's key
      "transfer_value": [u8; 32]    // 32-byte array key of encrypted amount
    }
    ```
    - **Notes**: 
      - Transfer only happens if sender has sufficient balance
      - If insufficient, a zero value is transferred
    - **Response**: 200 OK on success

## View (Decrypt)
    - **Endpoint**: `POST /decrypt`
    - **Description**: Decrypts and returns the value at the given key
    - **Request Body**:
    ```json
    {
      "key": [u8; 32]         // 32-byte array key of value to decrypt
    }
    ```
    - **Response**:
    ```json
    {
      "result": 1000           // Decrypted u64 value
    }
    ```

## Withdraw
    - **Endpoint**: `POST /withdraw`
    - **Description**: Withdraws value from an account and returns new balance
    - **Request Body**:
    ```json
    {
      "key": [u8; 32],        // 32-byte array key of account
      "value": [u8; 32]       // 32-byte array key of withdrawal amount
    }
    ```
    - **Notes**:
      - Withdrawal only happens if account has sufficient balance
      - If insufficient, a zero value is withdrawn
    - **Response**:
    ```json
    {
      "result": 900            // New decrypted balance after withdrawal
    }
    ```

## Error Responses
All endpoints may return the following error codes:
- 500 Internal Server Error: Indicates issues with encryption/decryption, serialization, or storage operations.