# Firebase
Firebase firestore implementations that use Firebase REST API.

## Requires
- Cloud Firestore REST API
### Endpoints
```
# Token endpoint
https://www.googleapis.com/auth/datastore
# Request endpoint
https://firestore.googleapis.com/v1/
```

## Features
- Use Firestore API

## Set up
1. Create Google service account
2. Set IAM
3. Call from your application


## Example
```
extern crate firebase;
use firebase::Firestore;
```

## APIs
### Read
#### Parameters
- order_by
- limit_to_first
- limit_to_last
- start_at
- end_at
- equal_to
- shallow

### Write
### Update
### Remove

