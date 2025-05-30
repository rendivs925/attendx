# Register user with Free plan
POST http://localhost:8000/auth/register
Content-Type: application/json
Accept-Language: en

{
  "name": "User D",
  "email": "userd@gmail.com",
  "password": "Securepassword123.",
  "password_confirmation": "Securepassword123.",
  "subscription_plan": "Free"
}

# Login as User A
POST http://localhost:8000/auth/login
Content-Type: application/json
Accept-Language: en

{
  "email": "usera@gmail.com",
  "password": "Securepassword123."
}

# Create organization (limits should be inferred from subscription plan)
POST http://localhost:8000/organizations/new
Content-Type: application/json
Accept-Language: en
Cookie: jwt_token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJuYW1lIjoiVXNlciBBIiwiZW1haWwiOiJ1c2VyYUBnbWFpbC5jb20iLCJleHAiOjE3NDg0NDMyMTh9.yjj8ioV3lRklcFETe590yO2W0E_TiTBzkkHSl0TOSIM

{
  "owner_id": "6835cd531698f3c7ad384dc8",
  "name": "Org Free Plan",
  "logo_url": "jsjsjsjsjsjsj",
  "email": "orgfree@example.com",
  "password": "OrgPass123."
}

# Register user with Pro plan
POST http://localhost:8000/auth/register
Content-Type: application/json
Accept-Language: en

{
  "name": "User B",
  "email": "userb@gmail.com",
  "password": "Securepassword123.",
  "password_confirmation": "Securepassword123.",
  "subscription_plan": "Pro"
}

# Login as User B
POST http://localhost:8000/auth/login
Content-Type: application/json
Accept-Language: en

{
  "email": "userb@gmail.com",
  "password": "Securepassword123."
}

# Create organization for Pro user
POST http://localhost:8000/organizations/new
Content-Type: application/json
Accept-Language: en
Cookie: jwt_token=<your_token_here>

{
  "name": "Org Pro Plan",
  "email": "orgpro@example.com",
  "logo_url": "jsjsjsj",
  "password": "OrgPass123."
}

# Get all organizations
GET http://localhost:8000/organizations/all
Accept-Language: en

# Get organization by ID (replace with actual ID)
GET http://localhost:8000/organizations/64a9d78fbc2a2a2a2a2a2a2a
Accept-Language: en

# Update organization (replace ID)
PUT http://localhost:8000/organizations/64a9d78fbc2a2a2a2a2a2a2a
Content-Type: application/json
Accept-Language: en

{
  "name": "Org Pro Plan Updated",
  "email": "orgpro-updated@example.com",
  "logo_url": "https://example.com/logo.png"
}

# Delete organization (replace ID)
DELETE http://localhost:8000/organizations/64a9d78fbc2a2a2a2a2a2a2a
Accept-Language: en

