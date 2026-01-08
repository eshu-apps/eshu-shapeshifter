# Gumroad Product Configuration - Eshu Shapeshifter

## 📋 Product Overview

**Product Name:** Eshu Shapeshifter - Linux Distribution Transformer
**Tagline:** Try Any Linux Distro. Keep Everything.
**Short URL:** `gumroad.com/l/eshu-shapeshifter`

---

## 🎯 Pricing Model

### Free Trial
- **2 FREE shapeshifts** included with download
- No credit card required for trial
- Full feature access during trial

### Subscription Option
- **$5.99/month** - Unlimited shapeshifts
- Cancel anytime
- Subscription duration: `monthly`
- Auto-renewal enabled

### Pay-Per-Use Option
- **$3.99 per pack** - 10 shapeshifts per purchase
- One-time payment
- No expiration
- Stackable (buy multiple packs)

---

## 📦 Gumroad Product Fields

### Basic Information
```
name: "Eshu Shapeshifter - Linux Distribution Transformer"
description: "Transform your Linux distribution without reinstalling. Try 2 shapeshifts free, then choose unlimited monthly access ($5.99/mo) or buy shifts in packs of 10 ($3.99). Seamlessly migrate between Arch, Ubuntu, Debian, Fedora, Kali, NixOS, and more while preserving all your data and configurations."

price: 0 (for free trial version)
currency: "USD"
custom_permalink: "eshu-shapeshifter"
published: true
```

### Pricing Structure

**Option 1: Free Trial Binary**
```json
{
  "name": "Eshu Shapeshifter (Free Trial)",
  "price": 0,
  "currency": "USD",
  "description": "Includes 2 free shapeshifts. Download and try risk-free.",
  "file_url": "https://github.com/eshu-apps/eshu-shapeshifter/releases/latest/download/eshu-shapeshifter-linux-x86_64.tar.gz",
  "tags": ["linux", "system", "free-trial"]
}
```

**Option 2: Monthly Subscription**
```json
{
  "name": "Eshu Shapeshifter - Unlimited Monthly",
  "price": 599,
  "currency": "USD",
  "subscription_duration": "monthly",
  "recurrences": null,
  "is_tiered_membership": false,
  "description": "Unlimited shapeshifts for $5.99/month. Cancel anytime.",
  "tags": ["linux", "subscription", "unlimited"]
}
```

**Option 3: Shift Pack (10 shifts)**
```json
{
  "name": "Eshu Shapeshifter - 10 Shift Pack",
  "price": 399,
  "currency": "USD",
  "description": "One-time purchase of 10 shapeshifts for $3.99. No expiration, stackable.",
  "max_purchase_count": 999,
  "tags": ["linux", "one-time", "pay-per-use"]
}
```

---

## 🔧 Variant Configuration

### Variants (Purchase Options)
```
Variant Category: "License Type"

Variants:
1. "2 Free Shifts (Trial)"
   - price_difference_cents: 0
   - max_purchase_count: 1 per customer

2. "Unlimited Monthly ($5.99/mo)"
   - price_difference_cents: 599
   - subscription_duration: "monthly"

3. "10 Shift Pack ($3.99)"
   - price_difference_cents: 399
   - max_purchase_count: 999
```

---

## 🎁 Offer Codes

### Launch Discount
```json
{
  "name": "LAUNCH2026",
  "amount_off": 200,
  "offer_type": "cents",
  "max_purchase_count": 1000,
  "universal": true
}
```
**Gives:** $2 off any purchase during launch period

### Beta Tester Discount
```json
{
  "name": "BETATESTER",
  "amount_off": 50,
  "offer_type": "percent",
  "max_purchase_count": 100,
  "universal": false
}
```
**Gives:** 50% off for first 100 beta testers

---

## 📝 Custom Fields (License Key Delivery)

### Field 1: License Key
```json
{
  "name": "License Key",
  "required": false,
  "variant_id": null
}
```
**Purpose:** Delivered automatically after purchase via Gumroad license key system

### Field 2: Product Type
```json
{
  "name": "Product Type",
  "required": false,
  "default_value": "trial|subscription|pack"
}
```
**Purpose:** Identifies which pricing option was purchased

---

## 📋 Product Description (Full)

```markdown
# 🔮 Eshu Shapeshifter - Try Any Linux Distro Without Reinstalling

**Transform your Linux distribution seamlessly while keeping all your data, applications, and configurations.**

## 🎁 Pricing Options

### 🆓 Free Trial
- **2 FREE shapeshifts** to test the tool
- No credit card required
- Full feature access

### 💎 Choose Your Plan

**Option 1: Unlimited Monthly - $5.99/month**
- ♾️ Unlimited shapeshifts
- 🔄 Try as many distros as you want
- ❌ Cancel anytime
- 💪 Best for distro hoppers & developers

**Option 2: Shift Packs - $3.99 per pack**
- 📦 10 shapeshifts per purchase
- 💰 One-time payment
- ⏰ Never expires
- 📚 Stack multiple packs
- 🎯 Best for occasional users

---

## ⚡ What You Get

- 🔄 Seamless migration between major Linux distributions
- 📸 Automatic snapshots before each transformation
- 🔙 One-click rollback if anything goes wrong
- 📦 Intelligent package translation (apt ↔ pacman ↔ dnf)
- ⚙️ Configuration preservation & translation
- 👥 Complete user data protection
- 🎨 Pre-configured beautiful desktop environments

## 🎯 Supported Distributions

**Featured Transformations:**
- 🔐 Kali Linux - Security & Pentesting
- 🌊 Hyprland - Beautiful Wayland compositor
- 🐉 Garuda Dragonized - Gaming & Performance
- ❄️ NixOS - Declarative & Reproducible
- 🚀 Pop!_OS COSMIC - Next-gen Rust desktop

**Standard Distributions:**
- Arch Linux
- Ubuntu 22.04+
- Debian 12+
- Fedora 39+
- openSUSE Tumbleweed

## 🛡️ Safety Features

- ✅ Automatic system snapshots (btrfs/LVM/rsync)
- ✅ Pre-migration validation checks
- ✅ Rollback capability
- ✅ Data preservation guaranteed
- ✅ Package compatibility checking

## 📋 Requirements

- Linux system (x86_64 or aarch64)
- Root/sudo access
- 20GB+ free disk space (for snapshots)
- Active internet connection

## 📥 Delivery

- **Instant download** after purchase
- Linux binary (x86_64)
- License key delivered via email
- Detailed documentation included
- Community support on GitHub

## 🔒 License

- **MIT License** - Open source
- **Usage tracking** for purchased shapeshifts
- **No telemetry** beyond license verification
- **Privacy-first** design

## 📞 Support

- 📖 Full documentation included
- 💬 GitHub community support
- 🐛 Bug reports via GitHub Issues
- 📧 Email: support@eshu-apps.com

---

**Named after Èṣù (pronounced "eh-SHOO"), the Yoruba orisha of crossroads and transformation.**

⚠️ **STATUS:** Beta - Coming Q2 2026

[Join the waitlist](https://eshuapps.com) • [View on GitHub](https://github.com/eshu-apps/eshu-shapeshifter) • [Documentation](https://github.com/eshu-apps/eshu-shapeshifter/tree/main/docs)
```

---

## 🔌 API Integration Details

### Creating Products via API

**Endpoint:** `POST https://api.gumroad.com/v2/products`

**Required Parameters:**
- `access_token` (your Gumroad API token)
- `name` (product name)
- `price` (in cents, 0 for free)

**Example Request (cURL):**
```bash
curl -X POST https://api.gumroad.com/v2/products \
  -d "access_token=YOUR_TOKEN" \
  -d "name=Eshu Shapeshifter - Unlimited Monthly" \
  -d "price=599" \
  -d "description=Unlimited shapeshifts for \$5.99/month" \
  -d "subscription_duration=monthly" \
  -d "custom_permalink=eshu-shapeshifter-unlimited"
```

### Editing Products via API

**Endpoint:** `PUT https://api.gumroad.com/v2/products/:id`

**Editable Fields:**
- `name`, `description`, `price`
- `subscription_duration`
- `custom_permalink`
- `tags` (comma-separated)

**Example Script (Python):**
```python
import requests

API_TOKEN = "your_gumroad_api_token"
PRODUCT_ID = "your_product_id"

def update_product_price(product_id, new_price_cents):
    url = f"https://api.gumroad.com/v2/products/{product_id}"
    data = {
        "access_token": API_TOKEN,
        "price": new_price_cents
    }
    response = requests.put(url, data=data)
    return response.json()

# Update subscription price to $7.99
result = update_product_price(PRODUCT_ID, 799)
print(result)
```

---

## 📊 Sales Tracking

### License Verification Webhook
**Endpoint:** `https://api.gumroad.com/v2/licenses/verify`

**Parameters:**
- `product_permalink` (e.g., "eshu-shapeshifter-unlimited")
- `license_key` (customer's key)
- `increment_uses_count` (boolean, default false)

**Response Fields:**
- `success` (boolean)
- `purchase.subscription_ended_at` (if subscription)
- `purchase.variants` (identifies which option purchased)
- `uses` (for pay-per-use packs)

**Example Verification:**
```bash
curl -X POST https://api.gumroad.com/v2/licenses/verify \
  -d "product_permalink=eshu-shapeshifter-pack" \
  -d "license_key=CUSTOMER_LICENSE_KEY" \
  -d "increment_uses_count=false"
```

---

## 🎨 Product Images

### Thumbnail Requirements
- **Size:** 800x600px minimum
- **Format:** PNG or JPG
- **Aspect ratio:** 4:3 or 16:9

### Cover Image
- **Size:** 1920x1080px recommended
- **Format:** PNG or JPG
- **Content:** Screenshot of shapeshifter in action

---

## 📝 Additional Notes

### Tags (for Gumroad search)
```
linux, distribution, distro, arch, ubuntu, debian, fedora, migration,
transformation, system-tools, developer-tools, devops, sysadmin,
open-source, rust, package-manager
```

### Categories
- Software > System Utilities
- Software > Developer Tools
- Software > Linux

### Content Warnings
- Requires root/administrator access
- Modifies system files
- Reboot required after transformation
- Recommended for experienced Linux users

---

## 🔗 External Links

- **GitHub Repository:** https://github.com/eshu-apps/eshu-shapeshifter
- **Documentation:** https://github.com/eshu-apps/eshu-shapeshifter/tree/main/docs
- **Website:** https://eshuapps.com
- **Support:** support@eshu-apps.com

---

## 🚀 Launch Checklist

- [ ] Create free trial product (price: $0)
- [ ] Create subscription product ($5.99/month)
- [ ] Create shift pack product ($3.99)
- [ ] Set up license key delivery
- [ ] Configure custom fields
- [ ] Add launch offer code (LAUNCH2026)
- [ ] Upload product images
- [ ] Write complete product description
- [ ] Test license verification
- [ ] Set up webhook for usage tracking

---

**Last Updated:** 2026-01-08
**API Version:** v2
**Status:** Ready for launch

---

## 📚 References

- [Gumroad API Documentation](https://gumroad.com/api)
- [Gumroad API Help Center](https://gumroad.com/help/article/280-create-application-api)
- [License Verification Guide](https://gumroad.com/api#licenses-verify-a-license)
