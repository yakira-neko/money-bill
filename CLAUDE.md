# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a personal finance management application built with Tauri + Vue 3 + TypeScript. It allows users to track income, expenses, assets, and liabilities through a desktop application with a SQLite backend.

## Architecture

- **Frontend**: Vue 3 with TypeScript and Tailwind CSS
- **Backend**: Rust with Tauri framework
- **Database**: SQLite with rusqlite
- **State Management**: Vue's reactive system
- **Internationalization**: vue-i18n with JSON language files
- **Build System**: Vite with pnpm

### Key Components

1. **Frontend Structure**:
   - `src/App.vue`: Main application component with sidebar navigation
   - `src/router.ts`: Vue Router configuration with routes for home, add bill, assets, budget, and history
   - `src/components/PC/`: Main components organized by view
   - `src/assets/i18n/`: Internationalization JSON files

2. **Backend Structure**:
   - `src-tauri/src/lib.rs`: Main Rust library with Tauri command handlers
   - `src-tauri/src/database/`: Database modules for accounts, transactions, and details
   - `src-tauri/src/database/init.rs`: Database schema initialization
   - SQLite database stored alongside the executable as `db.db3`

3. **Data Model**:
   - **ACCOUNT table**: Stores account information (name, currency, balance, icon, extra)
   - **TRANS table**: Stores transaction information (id, time, extra)
   - **DETAIL table**: Stores transaction details (id, trans_id, account, currency, balance)

## Common Development Tasks

### Running the Application

- `pnpm dev`: Start development server
- `pnpm build`: Build the application for production
- `pnpm tauri dev`: Run the Tauri application in development mode
- `pnpm tauri build`: Build the Tauri application for distribution

### Database Schema

The application uses a triple-entry bookkeeping system:
1. **ACCOUNT**: Stores account information
2. **TRANS**: Stores transaction headers
3. **DETAIL**: Stores transaction details linking accounts to transactions

Account names follow a hierarchical naming convention:
- Income accounts: `income::category::subcategory`
- Expense accounts: `expenses::category::subcategory`
- Asset accounts: `assets::category::subcategory`
- Liability accounts: `liabilities::category::subcategory`

### Key Tauri Commands

- `add_bills`: Add a new transaction with multiple account entries
- `get_income_accounts`, `get_expenses_accounts`, `get_assets_accounts`, `get_liabilities_accounts`: Retrieve account lists
- `get_income_expenses`: Get current month's income and expenses
- `get_weekly_income_expenses`: Get current week's income and expenses
- `get_transaction_history`: Retrieve transaction history

### Frontend Components

- `Home`: Dashboard with income/expense summary and weekly histogram
- `AddBill`: Form for adding new transactions
- `AssetsView`: Tree view of all accounts with balances
- `HistoryView`: List of all transactions with expandable details
- `BudgetView`: Budget management (incomplete)

## Testing

Tests are implemented in Rust using the standard testing framework with in-memory SQLite databases for unit tests.

## Internationalization

The application supports multiple languages through vue-i18n with JSON files in `src/assets/i18n/`.
