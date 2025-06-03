# Create organization member
POST http://localhost:8000/organization-members/new
Content-Type: application/json
Accept-Language: en
Cookie: jwt_token=<token_from_login>

{
  "organization_id": "<org_id>",
  "name": "John Doe",
  "role": "student",
  "identifiers": {
    "email": "johndoe@example.com"
  }
}

# Get all organization members
GET http://localhost:8000/organization-members/all/jajajajajajaj
Accept-Language: en
Cookie: jwt_token=jssjssjsjsjsjj

# Get specific member by org_id and member_id
GET http://localhost:8000/organization-members/<org_id>/<member_id>
Accept-Language: en
Cookie: jwt_token=<token_from_login>

# Update member by org_id and member_id
PUT http://localhost:8000/organization-members/<org_id>/<member_id>
Content-Type: application/json
Accept-Language: en
Cookie: jwt_token=<token_from_login>

{
  "organization_id": "<org_id>",
  "name": "John Updated",
  "role": "Member",
  "identifiers": {
    "email": "john_updated@example.com"
  }
}

# Delete member by org_id and member_id
DELETE http://localhost:8000/organization-members/<org_id>/<member_id>
Accept-Language: en
Cookie: jwt_token=<token_from_login>
