# In-App Purchases Module Implementation

## Overview

Comprehensive payment and monetization system demonstrating in-app purchase flows for mobile platforms (iOS/Android) and desktop licensing solutions. Includes mock/sandbox implementation for testing without real payment infrastructure.

## Current Implementation Status

✅ **Mock/Sandbox Mode** - Fully functional demonstration system
⚠️ **Production Ready** - Architecture ready, requires platform plugins

## Plugin Setup

### Desktop Self-Distribution (Recommended)

For desktop apps distributed outside app stores, use **License Keys + Payment Provider**:

#### Step 1: License Key System

Choose one of these Tauri-specific solutions:

```bash
# Option 1: Keyforge (Tauri-specific, Stripe integration)
# https://keyforge.dev

# Option 2: Keygen (Tauri + Electron, feature entitlements)
# https://keygen.sh

# Option 3: Anystack (License validation + auto-updates)
# https://anystack.sh
```

#### Step 2: Payment Provider

**Recommended Providers:**

```bash
# Option A: Stripe (Industry standard - 2.9% + $0.30)
bun add stripe

# Option B: Lemon Squeezy (MoR - 5%, handles tax/VAT)
# Use their API directly

# Option C: Polar.sh (Developer-focused - 4% + $0.40)
# Use their API directly

# Option D: Dodo Payments (Modern SaaS billing)
bun add @dodopayments/sdk
```

### Mobile App Store Distribution

#### iOS (App Store)
```bash
# Install StoreKit plugin
bun add tauri-plugin-iap
```

```toml
# Add to src-tauri/Cargo.toml
[dependencies]
tauri-plugin-iap = "2.0"  # iOS-specific
```

#### Android (Play Store)
```bash
# Install in-app purchase plugin
bun add tauri-plugin-in-app-purchase
```

```toml
# Add to src-tauri/Cargo.toml
[dependencies]
tauri-plugin-in-app-purchase = "2.0"  # Android + iOS + Windows
```

### Desktop (Traditional Software Vendors)

For traditional desktop software sales:

- **FastSpring** - 4.9% + $0.49, excellent license management
- **Paddle** - 5% + $0.50, developer SDKs, in-app purchases

Both are Merchant of Record (MoR) and handle tax, fraud, compliance.

## Core Features

### Current Mock/Sandbox Features
- [x] Platform detection (iOS, Android, Windows, macOS, Linux)
- [x] Product catalog management
- [x] Purchase flow simulation
- [x] Transaction ID generation
- [x] Receipt validation
- [x] Purchase restoration
- [x] Error handling (10% failure simulation)
- [x] Multi-currency support
- [x] Product types (consumable, non-consumable, subscription)
- [x] Realistic delays (network simulation)

### Production Features (Plugin Integration Required)
- [ ] Real App Store / Play Store product fetching
- [ ] Platform-native purchase UI
- [ ] Receipt verification with Apple/Google servers
- [ ] Subscription management
- [ ] Server-side receipt validation
- [ ] Webhook integration for purchase events
- [ ] License key generation (desktop)
- [ ] Offline license validation (desktop)

## Mock Data Structures

### Product Schema
```typescript
interface Product {
  id: string              // "premium_monthly"
  title: string           // "Premium Monthly"
  description: string     // Feature description
  price: string           // "$9.99" (formatted)
  price_amount: number    // 9.99 (numeric)
  currency: string        // "USD"
  type: "consumable" | "non-consumable" | "subscription"
}
```

### Receipt Schema
```typescript
interface PurchaseReceipt {
  product_id: string      // Product identifier
  transaction_id: string  // "txn_1234567890_premium"
  purchase_date: string   // ISO 8601 timestamp
  status: string          // "completed", "pending", "failed"
  platform: string        // "ios", "android", "macos", etc.
  price_paid: number      // 9.99
  currency: string        // "USD"
}
```

### Restore Result Schema
```typescript
interface RestoreResult {
  restored_count: number
  receipts: PurchaseReceipt[]
}
```

## Rust Backend

### Mock Implementation

Current implementation in `src-tauri/src/lib.rs`:

#### 1. Fetch Products
```rust
#[tauri::command]
async fn fetch_iap_products() -> Result<Vec<IapProduct>, String> {
    // Simulates network delay (800ms)
    tokio::time::sleep(Duration::from_millis(800)).await;

    // Returns 5 mock products:
    // - Premium Monthly ($9.99)
    // - Premium Yearly ($99.99)
    // - 100 Coins ($4.99)
    // - 500 Coins ($19.99)
    // - Remove Ads ($2.99)

    Ok(vec![/* ... */])
}
```

#### 2. Purchase Product
```rust
#[tauri::command]
async fn purchase_product(product_id: String) -> Result<PurchaseReceipt, String> {
    // Simulates purchase flow delay (2000ms)
    tokio::time::sleep(Duration::from_millis(2000)).await;

    // Validates product exists
    let product = fetch_iap_products().await?
        .iter()
        .find(|p| p.id == product_id)
        .ok_or("Product not found")?;

    // Simulates 10% failure rate
    if timestamp % 10 == 0 {
        return Err("Purchase cancelled by user".to_string());
    }

    // Generates unique transaction ID
    let transaction_id = format!("txn_{}_{}", timestamp, product_id);

    // Returns receipt with purchase details
    Ok(PurchaseReceipt { /* ... */ })
}
```

#### 3. Restore Purchases
```rust
#[tauri::command]
async fn restore_purchases() -> Result<RestoreResult, String> {
    // Simulates platform restore delay (1500ms)
    tokio::time::sleep(Duration::from_millis(1500)).await;

    // Returns mock previous purchases
    // In production: queries App Store/Play Store
    Ok(RestoreResult {
        restored_count: 1,
        receipts: vec![/* ... */]
    })
}
```

#### 4. Validate Receipt
```rust
#[tauri::command]
async fn validate_receipt(transaction_id: String) -> Result<bool, String> {
    // Simulates validation delay (800ms)
    tokio::time::sleep(Duration::from_millis(800)).await;

    // Mock validation: checks transaction ID format
    // In production: sends to backend → verifies with Apple/Google
    Ok(transaction_id.starts_with("txn_"))
}
```

#### 5. Platform Detection
```rust
#[tauri::command]
fn get_iap_platform() -> String {
    #[cfg(target_os = "ios")] return "ios".to_string();
    #[cfg(target_os = "android")] return "android".to_string();
    #[cfg(target_os = "windows")] return "windows".to_string();
    #[cfg(target_os = "macos")] return "macos".to_string();
    #[cfg(target_os = "linux")] return "linux".to_string();
    "unknown".to_string()
}
```

## Frontend Integration

### React Component Structure

```typescript
// State management
const [products, setProducts] = useState<Product[]>([])
const [receipts, setReceipts] = useState<PurchaseReceipt[]>([])
const [platform, setPlatform] = useState<string>('unknown')
const [loading, setLoading] = useState<string | null>(null)

// Platform detection on mount
useEffect(() => {
  const platformName = await invoke<string>('get_iap_platform')
  setPlatform(platformName)
}, [])

// Fetch products
const fetchedProducts = await invoke<Product[]>('fetch_iap_products')

// Purchase flow
const receipt = await invoke<PurchaseReceipt>('purchase_product', {
  productId: 'premium_monthly'
})

// Restore purchases
const result = await invoke<RestoreResult>('restore_purchases')

// Validate receipt
const isValid = await invoke<boolean>('validate_receipt', {
  transactionId: 'txn_123'
})
```

## Security Best Practices

### Client-Side
- ✅ Never expose API keys in client code
- ✅ Use environment variables for sensitive data
- ✅ Validate all user input
- ✅ Store licenses securely using `tauri-plugin-stronghold` or Keychain
- ✅ Implement certificate pinning for API calls

### Server-Side
- ✅ Process all payments server-side
- ✅ Validate receipts with Apple/Google servers
- ✅ Implement webhook verification
- ✅ Use HTTPS for all API calls
- ✅ Log all transactions for auditing
- ✅ Implement rate limiting
- ✅ Handle payment disputes/refunds

### Platform-Specific

#### iOS (App Store)
- Must use StoreKit for purchases
- No third-party payments allowed in App Store
- Requires App Store Connect configuration
- Product IDs must match App Store Connect
- Receipt validation via Apple servers

#### Android (Play Store)
- Must use Google Play Billing
- Requires Google Play Console setup
- Product SKUs must match console
- Receipt validation via Google servers

#### Desktop (Self-Distributed)
- Use any payment method
- Implement license key system
- Consider offline activation
- Store licenses securely
- Handle license transfers

## Production Migration Guide

### For Mobile (iOS/Android)

1. **Install Platform Plugin**
```bash
bun add tauri-plugin-in-app-purchase
```

2. **Configure App Store Connect / Play Console**
   - Create products/subscriptions
   - Set pricing
   - Configure tax settings
   - Enable billing

3. **Update Rust Commands**
```rust
// Replace mock with real plugin calls
use tauri_plugin_in_app_purchase::*;

#[tauri::command]
async fn fetch_iap_products() -> Result<Vec<Product>, String> {
    // Use real plugin
    InAppPurchase::get_products(product_ids).await
}
```

4. **Implement Receipt Validation Backend**
```rust
// Send receipts to your server for validation
async fn validate_receipt_server(receipt: String) -> Result<bool, String> {
    // POST to your backend
    // Backend verifies with Apple/Google
    // Returns validation result
}
```

### For Desktop (License Keys)

1. **Choose License System**
   - Keyforge + Stripe
   - Keygen + Any payment provider
   - Anystack + Any payment provider

2. **Implement License Validation**
```rust
use keyforge::*; // or keygen, anystack

#[tauri::command]
async fn activate_license(key: String) -> Result<License, String> {
    // Validate with license server
    let license = Keyforge::validate(key).await?;

    // Store securely
    stronghold::store("license", license).await?;

    Ok(license)
}
```

3. **Integrate Payment Provider**
```typescript
// Frontend: Redirect to Stripe Checkout
const session = await createCheckoutSession({
  price_id: 'price_xxx',
  success_url: 'myapp://success',
  cancel_url: 'myapp://cancel'
})
window.location.href = session.url

// Backend: Generate license on successful payment
stripe.webhooks.handleEvent('checkout.session.completed', async (event) => {
  const license = await keyforge.createLicense({
    product: event.product,
    customer: event.customer
  })
  await sendEmailWithLicense(license)
})
```

## Payment Provider Comparison

### Stripe (Recommended for Custom Integration)
- **Rate**: 2.9% + $0.30 per transaction
- **Best for**: Maximum control, custom integration
- **Pros**: Industry standard, extensive API, global support
- **Cons**: Requires more setup, handle tax yourself
- **Works with**: All license systems

### Lemon Squeezy (Recommended for Indie Devs)
- **Rate**: 5% (all-inclusive)
- **Best for**: Quick setup, indie developers
- **Pros**: Merchant of Record, handles tax/VAT/compliance
- **Cons**: Higher fees, less control
- **Acquired**: Stripe (2024), still independent

### Polar.sh (Recommended for Open Source)
- **Rate**: 4% + $0.40 per transaction
- **Best for**: Developers, open-source projects
- **Pros**: Transparent pricing, powerful API/CLI
- **Cons**: Newer platform, smaller ecosystem

### Dodo Payments (Recommended for SaaS)
- **Rate**: Variable (usage-based pricing available)
- **Best for**: SaaS apps, AI products, usage billing
- **Pros**: Modern platform, AI assistant, multiple SDKs
- **Cons**: Less established than Stripe

### FastSpring (Traditional Desktop Software)
- **Rate**: 4.9% + $0.49
- **Best for**: Desktop software, license management
- **Pros**: Excellent licensing features, MoR
- **Cons**: Traditional approach, older platform

### Paddle (Traditional Desktop Software)
- **Rate**: 5% + $0.50
- **Best for**: Developer-focused products, SDKs
- **Pros**: Developer SDKs, MoR, in-app features
- **Cons**: Higher fees, complex setup

## Distribution Strategy

### Mac App Store
- ⚠️ **MUST** use Apple IAP (In-App Purchase)
- ⚠️ Third-party payments **NOT ALLOWED**
- Apple takes 15-30% commission
- Automatic tax handling

### Self-Distribution (Recommended)
- ✅ Use **ANY** payment method
- ✅ Lower fees (2.9% vs 30%)
- ✅ Direct customer relationship
- ✅ Flexible pricing
- ⚠️ Handle tax compliance yourself (or use MoR)
- ⚠️ Manual distribution/updates (use Tauri updater)

## Testing

### Mock/Sandbox Mode (Current)
- No real payments required
- Instant testing
- 10% failure rate for error testing
- All features functional
- Safe for development

### Sandbox Testing (Production Plugin)

#### iOS Sandbox
```bash
# Use sandbox Apple ID
# Test purchases don't charge
# Receipt validation against sandbox server
```

#### Android Testing
```bash
# Use test Google account
# License testing tracks
# No real charges
```

### Desktop Testing
```bash
# Use Stripe test mode
# Test credit cards (4242 4242 4242 4242)
# Webhook testing with Stripe CLI
stripe listen --forward-to localhost:3000/webhooks
```

## Troubleshooting

### Common Issues

**Products not loading**
- Check internet connection
- Verify product IDs match store configuration
- Check permissions in capabilities

**Purchase fails**
- Check platform supports IAP
- Verify user is signed in to store
- Check payment method is valid
- Review error messages in console

**Receipt validation fails**
- Ensure receipt is recent
- Check server connectivity
- Verify using correct environment (sandbox vs production)

**License activation fails (Desktop)**
- Check internet connection
- Verify license key format
- Ensure license hasn't been deactivated
- Check activation limit

## Resources

### Official Documentation
- [Tauri Plugin System](https://tauri.app/v2/develop/plugins/)
- [StoreKit (iOS)](https://developer.apple.com/storekit/)
- [Google Play Billing](https://developer.android.com/google/play/billing)
- [Stripe Documentation](https://stripe.com/docs)

### License Systems
- [Keyforge](https://keyforge.dev) - Tauri-specific
- [Keygen](https://keygen.sh) - Tauri + Electron
- [Anystack](https://anystack.sh) - License + updates

### Payment Providers
- [Stripe](https://stripe.com) - Industry standard
- [Lemon Squeezy](https://lemonsqueezy.com) - MoR for indie devs
- [Polar.sh](https://polar.sh) - Developer-focused
- [Dodo Payments](https://dodopayments.com) - Modern SaaS
- [FastSpring](https://fastspring.com) - Desktop software
- [Paddle](https://paddle.com) - Developer products

## Next Steps

1. **Choose Distribution Method**
   - App Store: Use platform IAP
   - Self-distributed: Use license keys + payment provider

2. **Select Payment Provider**
   - Start with Stripe for flexibility
   - Consider Lemon Squeezy for simplicity
   - Polar.sh for open-source projects

3. **Implement Backend**
   - Receipt validation endpoint
   - License generation (desktop)
   - Webhook handlers

4. **Test Thoroughly**
   - Sandbox mode first
   - Test all purchase flows
   - Verify receipt validation
   - Test restoration

5. **Go Live**
   - Switch to production keys
   - Monitor transactions
   - Handle support requests
   - Track revenue

## Legal Considerations

- ⚠️ Comply with platform policies (Apple/Google)
- ⚠️ Display clear pricing and terms
- ⚠️ Handle refunds appropriately
- ⚠️ Collect required tax information
- ⚠️ Implement GDPR compliance (EU)
- ⚠️ Follow PCI DSS for payment data
- ⚠️ Provide clear privacy policy
- ⚠️ Honor subscription cancellations

## Support

For issues specific to this implementation:
1. Check console logs for errors
2. Verify platform permissions
3. Test in sandbox mode first
4. Review provider documentation
5. Check Tauri Discord for help

---

**Last Updated**: November 2025
**Module Version**: 1.0.0
**Status**: Mock/Sandbox Implementation Complete ✅
