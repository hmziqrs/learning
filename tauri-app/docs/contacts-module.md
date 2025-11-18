# Contacts Module Implementation

## Overview

Access device contacts with read capabilities, allowing users to view and search through their contact list. This module demonstrates native integration for accessing contact data from Android, iOS, and macOS devices using platform-specific APIs.

## Current Implementation Status

⚠️ **Planned** - Custom plugin development required

## Plugin Requirements

### No Official Plugin Available

Tauri does not provide an official contacts plugin. Implementation requires building a custom mobile plugin.

**Options**:
- Build custom Tauri mobile plugin (Recommended)
- Use community plugins (if available and maintained)

## Permissions Configuration

### Android Permissions

Add to `src-tauri/gen/android/app/src/main/AndroidManifest.xml`:

```xml
<uses-permission android:name="android.permission.READ_CONTACTS" />
<uses-permission android:name="android.permission.WRITE_CONTACTS" /> <!-- Optional -->
```

### iOS/macOS Permissions

Add to `src-tauri/gen/apple/Info.plist` (iOS) or app's `Info.plist` (macOS):

```xml
<key>NSContactsUsageDescription</key>
<string>This app needs access to your contacts to display and manage them.</string>
```

**Note**: Both iOS and macOS use the same Contacts framework (`CNContactStore`), so the permission handling is identical.

### Tauri Capabilities

Add custom permissions to `src-tauri/capabilities/default.json`:

```json
{
  "permissions": [
    "contacts:default",
    "contacts:allow-read-contacts",
    "contacts:allow-check-permission",
    "contacts:allow-request-permission"
  ]
}
```

## Core Features

### Permission Management
- [ ] Check if contacts permission is granted
- [ ] Request contacts permission
- [ ] Display permission status
- [ ] Handle permission denial

### Contact Access
- [ ] Fetch all contacts from device
- [ ] Read contact name
- [ ] Read contact phone numbers
- [ ] Read contact email addresses
- [ ] Read contact profile photo (optional)

### Contact Operations
- [ ] Search/filter contacts by name
- [ ] Search by phone number
- [ ] Search by email
- [ ] Display contact count
- [ ] Handle empty contact list

### UI Features
- [ ] Display contacts in list format
- [ ] Show contact details
- [ ] Implement search functionality
- [ ] Show loading states
- [ ] Handle errors gracefully

## Data Structures

### Contact Schema

```typescript
interface Contact {
  id: string
  name: string
  phoneNumbers: PhoneNumber[]
  emails: Email[]
  photoUri?: string
}

interface PhoneNumber {
  type: 'mobile' | 'home' | 'work' | 'other'
  number: string
}

interface Email {
  type: 'personal' | 'work' | 'other'
  address: string
}
```

### Permission Status Schema

```typescript
interface PermissionStatus {
  granted: boolean
  canRequest: boolean
  message?: string
}
```

## Custom Plugin Development

### Android Implementation (Kotlin)

Create custom plugin in `src-tauri/gen/android/`:

```kotlin
import android.Manifest
import android.content.ContentResolver
import android.content.pm.PackageManager
import android.provider.ContactsContract
import androidx.core.content.ContextCompat
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@TauriPlugin
class ContactsPlugin(private val activity: Activity) : Plugin(activity) {

    @Command
    fun checkPermission(): Boolean {
        return ContextCompat.checkSelfPermission(
            activity,
            Manifest.permission.READ_CONTACTS
        ) == PackageManager.PERMISSION_GRANTED
    }

    @Command
    fun requestPermission() {
        ActivityCompat.requestPermissions(
            activity,
            arrayOf(Manifest.permission.READ_CONTACTS),
            REQUEST_CODE_READ_CONTACTS
        )
    }

    @Command
    fun getContacts(): List<JSObject> {
        val contacts = mutableListOf<JSObject>()
        val contentResolver: ContentResolver = activity.contentResolver

        val cursor = contentResolver.query(
            ContactsContract.Contacts.CONTENT_URI,
            null,
            null,
            null,
            ContactsContract.Contacts.DISPLAY_NAME + " ASC"
        )

        cursor?.use {
            while (it.moveToNext()) {
                val id = it.getString(it.getColumnIndex(ContactsContract.Contacts._ID))
                val name = it.getString(it.getColumnIndex(ContactsContract.Contacts.DISPLAY_NAME))

                val contact = JSObject()
                contact.put("id", id)
                contact.put("name", name)
                contact.put("phoneNumbers", getPhoneNumbers(contentResolver, id))
                contact.put("emails", getEmails(contentResolver, id))

                contacts.add(contact)
            }
        }

        return contacts
    }

    private fun getPhoneNumbers(contentResolver: ContentResolver, contactId: String): List<JSObject> {
        val phoneNumbers = mutableListOf<JSObject>()
        val cursor = contentResolver.query(
            ContactsContract.CommonDataKinds.Phone.CONTENT_URI,
            null,
            ContactsContract.CommonDataKinds.Phone.CONTACT_ID + " = ?",
            arrayOf(contactId),
            null
        )

        cursor?.use {
            while (it.moveToNext()) {
                val number = it.getString(it.getColumnIndex(ContactsContract.CommonDataKinds.Phone.NUMBER))
                val type = it.getInt(it.getColumnIndex(ContactsContract.CommonDataKinds.Phone.TYPE))

                val phone = JSObject()
                phone.put("number", number)
                phone.put("type", getPhoneType(type))

                phoneNumbers.add(phone)
            }
        }

        return phoneNumbers
    }

    private fun getEmails(contentResolver: ContentResolver, contactId: String): List<JSObject> {
        val emails = mutableListOf<JSObject>()
        val cursor = contentResolver.query(
            ContactsContract.CommonDataKinds.Email.CONTENT_URI,
            null,
            ContactsContract.CommonDataKinds.Email.CONTACT_ID + " = ?",
            arrayOf(contactId),
            null
        )

        cursor?.use {
            while (it.moveToNext()) {
                val address = it.getString(it.getColumnIndex(ContactsContract.CommonDataKinds.Email.ADDRESS))
                val type = it.getInt(it.getColumnIndex(ContactsContract.CommonDataKinds.Email.TYPE))

                val email = JSObject()
                email.put("address", address)
                email.put("type", getEmailType(type))

                emails.add(email)
            }
        }

        return emails
    }

    private fun getPhoneType(type: Int): String {
        return when (type) {
            ContactsContract.CommonDataKinds.Phone.TYPE_MOBILE -> "mobile"
            ContactsContract.CommonDataKinds.Phone.TYPE_HOME -> "home"
            ContactsContract.CommonDataKinds.Phone.TYPE_WORK -> "work"
            else -> "other"
        }
    }

    private fun getEmailType(type: Int): String {
        return when (type) {
            ContactsContract.CommonDataKinds.Email.TYPE_HOME -> "personal"
            ContactsContract.CommonDataKinds.Email.TYPE_WORK -> "work"
            else -> "other"
        }
    }

    companion object {
        private const val REQUEST_CODE_READ_CONTACTS = 100
    }
}
```

### iOS/macOS Implementation (Swift)

Create custom plugin in `src-tauri/gen/apple/` (works for both iOS and macOS):

```swift
import Contacts
import Tauri
import UIKit
import WebKit

class ContactsPlugin: Plugin {
    @objc public func checkPermission(_ invoke: Invoke) {
        let authorizationStatus = CNContactStore.authorizationStatus(for: .contacts)
        invoke.resolve(["granted": authorizationStatus == .authorized])
    }

    @objc public func requestPermission(_ invoke: Invoke) {
        let store = CNContactStore()
        store.requestAccess(for: .contacts) { granted, error in
            if let error = error {
                invoke.reject(error.localizedDescription)
            } else {
                invoke.resolve(["granted": granted])
            }
        }
    }

    @objc public func getContacts(_ invoke: Invoke) {
        let store = CNContactStore()
        let keysToFetch = [
            CNContactGivenNameKey,
            CNContactFamilyNameKey,
            CNContactPhoneNumbersKey,
            CNContactEmailAddressesKey,
            CNContactImageDataKey
        ] as [CNKeyDescriptor]

        let request = CNContactFetchRequest(keysToFetch: keysToFetch)
        var contacts: [[String: Any]] = []

        do {
            try store.enumerateContacts(with: request) { contact, _ in
                var contactDict: [String: Any] = [:]
                contactDict["id"] = contact.identifier
                contactDict["name"] = "\(contact.givenName) \(contact.familyName)".trimmingCharacters(in: .whitespaces)

                var phoneNumbers: [[String: String]] = []
                for phoneNumber in contact.phoneNumbers {
                    phoneNumbers.append([
                        "number": phoneNumber.value.stringValue,
                        "type": self.getPhoneType(phoneNumber.label)
                    ])
                }
                contactDict["phoneNumbers"] = phoneNumbers

                var emails: [[String: String]] = []
                for email in contact.emailAddresses {
                    emails.append([
                        "address": email.value as String,
                        "type": self.getEmailType(email.label)
                    ])
                }
                contactDict["emails"] = emails

                contacts.append(contactDict)
            }

            invoke.resolve(["contacts": contacts])
        } catch {
            invoke.reject(error.localizedDescription)
        }
    }

    private func getPhoneType(_ label: String?) -> String {
        guard let label = label else { return "other" }
        switch label {
        case CNLabelPhoneNumberMobile:
            return "mobile"
        case CNLabelHome:
            return "home"
        case CNLabelWork:
            return "work"
        default:
            return "other"
        }
    }

    private func getEmailType(_ label: String?) -> String {
        guard let label = label else { return "other" }
        switch label {
        case CNLabelHome:
            return "personal"
        case CNLabelWork:
            return "work"
        default:
            return "other"
        }
    }
}
```

## Rust Backend

### Tauri Commands

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    id: String,
    name: String,
    phone_numbers: Vec<PhoneNumber>,
    emails: Vec<Email>,
    photo_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    r#type: String,
    number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    r#type: String,
    address: String,
}

#[tauri::command]
async fn check_contacts_permission() -> Result<bool, String> {
    // Calls mobile plugin
    // Platform-specific implementation
    #[cfg(target_os = "android")]
    {
        // Android implementation via plugin
        Ok(false)
    }

    #[cfg(target_os = "ios")]
    {
        // iOS implementation via plugin
        Ok(false)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        Err("Contacts API is only available on mobile platforms".to_string())
    }
}

#[tauri::command]
async fn request_contacts_permission() -> Result<bool, String> {
    // Calls mobile plugin to request permission
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        // Mobile implementation via plugin
        Ok(false)
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        Err("Contacts API is only available on mobile platforms".to_string())
    }
}

#[tauri::command]
async fn get_contacts() -> Result<Vec<Contact>, String> {
    // Calls mobile plugin to fetch contacts
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        // Mobile implementation via plugin
        Ok(vec![])
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        Err("Contacts API is only available on mobile platforms".to_string())
    }
}
```

### Register Commands

Add to `src-tauri/src/lib.rs`:

```rust
.invoke_handler(tauri::generate_handler![
    check_contacts_permission,
    request_contacts_permission,
    get_contacts
])
```

## Frontend Implementation

### React Component Structure

```typescript
import { invoke } from '@tauri-apps/api/core'
import { useState, useEffect } from 'react'

interface Contact {
  id: string
  name: string
  phoneNumbers: Array<{ type: string; number: string }>
  emails: Array<{ type: string; address: string }>
  photoUri?: string
}

const ContactsPage = () => {
  const [contacts, setContacts] = useState<Contact[]>([])
  const [permissionGranted, setPermissionGranted] = useState(false)
  const [loading, setLoading] = useState(false)
  const [searchQuery, setSearchQuery] = useState('')

  useEffect(() => {
    checkPermission()
  }, [])

  const checkPermission = async () => {
    try {
      const granted = await invoke<boolean>('check_contacts_permission')
      setPermissionGranted(granted)
    } catch (error) {
      console.error('Failed to check permission:', error)
    }
  }

  const requestPermission = async () => {
    try {
      const granted = await invoke<boolean>('request_contacts_permission')
      setPermissionGranted(granted)
    } catch (error) {
      console.error('Failed to request permission:', error)
    }
  }

  const loadContacts = async () => {
    if (!permissionGranted) {
      await requestPermission()
      return
    }

    setLoading(true)
    try {
      const contactList = await invoke<Contact[]>('get_contacts')
      setContacts(contactList)
    } catch (error) {
      console.error('Failed to load contacts:', error)
    } finally {
      setLoading(false)
    }
  }

  const filteredContacts = contacts.filter(contact =>
    contact.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    contact.phoneNumbers.some(p => p.number.includes(searchQuery)) ||
    contact.emails.some(e => e.address.toLowerCase().includes(searchQuery.toLowerCase()))
  )

  return (
    <div>
      {/* Permission UI */}
      {!permissionGranted && (
        <button onClick={requestPermission}>
          Request Contacts Permission
        </button>
      )}

      {/* Load Contacts */}
      {permissionGranted && (
        <button onClick={loadContacts} disabled={loading}>
          {loading ? 'Loading...' : 'Load Contacts'}
        </button>
      )}

      {/* Search */}
      {contacts.length > 0 && (
        <input
          type="text"
          placeholder="Search contacts..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
        />
      )}

      {/* Contact List */}
      <div>
        {filteredContacts.map(contact => (
          <div key={contact.id}>
            <h3>{contact.name}</h3>
            {contact.phoneNumbers.map((phone, i) => (
              <p key={i}>{phone.type}: {phone.number}</p>
            ))}
            {contact.emails.map((email, i) => (
              <p key={i}>{email.type}: {email.address}</p>
            ))}
          </div>
        ))}
      </div>

      {/* Contact Count */}
      {contacts.length > 0 && (
        <p>Total: {contacts.length} contacts</p>
      )}
    </div>
  )
}
```

## UI Components

### Permission Section
- [ ] Permission status display
- [ ] Request permission button
- [ ] Permission explanation text
- [ ] Visual indicator (granted/denied)

### Contact Loading Section
- [ ] Load contacts button
- [ ] Loading indicator
- [ ] Contact count display
- [ ] Refresh button

### Search Section
- [ ] Search input field
- [ ] Search by name
- [ ] Search by phone
- [ ] Search by email
- [ ] Clear search button

### Contact List Section
- [ ] Contact card/list item
- [ ] Contact name display
- [ ] Phone numbers list
- [ ] Email addresses list
- [ ] Contact photo (optional)
- [ ] Empty state message

### Output Panel
- [ ] Display operation results
- [ ] Show success/error messages
- [ ] Display contact details

## Testing Checklist

### Android Testing
- [ ] Check permission on first launch
- [ ] Request permission flow
- [ ] Load contacts successfully
- [ ] Search contacts by name
- [ ] Search contacts by phone
- [ ] Search contacts by email
- [ ] Display contact details correctly
- [ ] Handle permission denial
- [ ] Handle empty contact list

### iOS Testing
- [ ] Check permission on first launch
- [ ] Request permission flow
- [ ] Load contacts successfully
- [ ] Search contacts by name
- [ ] Search contacts by phone
- [ ] Search contacts by email
- [ ] Display contact details correctly
- [ ] Handle permission denial
- [ ] Handle empty contact list

### Desktop Testing
- [ ] Show appropriate message (mobile-only feature)
- [ ] Graceful degradation

### Edge Cases
- [ ] Handle no contacts
- [ ] Handle contacts with no phone numbers
- [ ] Handle contacts with no email addresses
- [ ] Handle special characters in names
- [ ] Handle very long contact lists
- [ ] Handle duplicate contacts
- [ ] Test with poor performance devices

## Implementation Notes

### Platform Support

**Mobile + macOS**
- Android: Full support via ContactsContract API
- iOS: Full support via Contacts framework (CNContactStore)
- macOS: Full support via Contacts framework (CNContactStore) - same as iOS
- Windows/Linux: Not supported (show informative message)

### Privacy Considerations
- Always explain why contacts access is needed
- Request permission only when necessary
- Handle permission denial gracefully
- Never store contacts without user consent
- Follow platform privacy guidelines

### Performance Optimization
- Load contacts asynchronously
- Implement pagination for large contact lists
- Use virtualization for rendering long lists
- Cache contact data appropriately
- Debounce search input

### Best Practices
- Request permission with clear explanation
- Handle all permission states properly
- Show loading states during operations
- Provide search/filter functionality
- Display contact count
- Handle errors with user-friendly messages
- Test on various devices with different contact list sizes

## Progress Tracking

### Setup Phase
- [ ] Research custom plugin development
- [ ] Set up Android plugin structure
- [ ] Set up iOS plugin structure
- [ ] Configure permissions
- [ ] Create Rust bridge commands

### Development Phase
- [ ] Implement Android contact access
- [ ] Implement iOS contact access
- [ ] Implement permission checking
- [ ] Implement permission requesting
- [ ] Build UI components
- [ ] Add search functionality
- [ ] Add error handling
- [ ] Add loading states

### Testing Phase
- [ ] Test on Android devices
- [ ] Test on iOS devices
- [ ] Test permission flows
- [ ] Test with large contact lists
- [ ] Test edge cases
- [ ] Fix bugs

### Polish Phase
- [ ] Improve UI/UX
- [ ] Optimize performance
- [ ] Add better error messages
- [ ] Add success feedback
- [ ] Code cleanup and documentation

## Resources

### Android Documentation
- [ContactsContract API](https://developer.android.com/reference/android/provider/ContactsContract)
- [Request Runtime Permissions](https://developer.android.com/training/permissions/requesting)

### iOS Documentation
- [Contacts Framework](https://developer.apple.com/documentation/contacts)
- [CNContactStore](https://developer.apple.com/documentation/contacts/cncontactstore)

### Tauri Resources
- [Tauri Mobile Plugin Guide](https://v2.tauri.app/develop/plugins/)
- [Building Mobile Plugins](https://v2.tauri.app/develop/plugins/build-mobile-plugins/)

## Platform Support

| Feature | Windows | macOS | Linux | iOS | Android |
|---------|---------|-------|-------|-----|---------|
| Read Contacts | ❌ | ✅* | ❌ | ✅ | ✅ |
| Search Contacts | ❌ | ✅* | ❌ | ✅ | ✅ |
| Permission Check | ❌ | ✅* | ❌ | ✅ | ✅ |
| Permission Request | ❌ | ✅* | ❌ | ✅ | ✅ |

\* macOS uses the same Contacts framework as iOS (`CNContactStore`)

---

**Module Type**: Mobile + macOS (Custom Plugin Required)
**Status**: Planning Phase
**Supported Platforms**: iOS, Android, macOS
