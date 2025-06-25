POST http://localhost:8000/api/auth/login
Content-Type: application/json
Accept-Language: en

{
  "email": "tono@gmail.com",
  "password": "Securepjjjassword123."
}

DELETE http://localhost:8000/api/auth/logout
Accept-Language: ja

POST http://localhost:8000/api/auth/register
Content-Type: application/json
Accept-Language: en

{
  "name": "user",
  "email": "yuni@gmail.com",
  "password": "Securepassword123.",
  "password_confirmation": "Securepassword123.",
  "subscription_plan": "free",
  "role": "orgowner"
}

GET http://localhost:8000/api/users/all
Accept-Language: en

GET http://localhost:8000/api/users/h1@gmail.com
Accept-Language: id

DELETE http://localhost:8000/api/users/h1@gmail.com
Accept-Language: en

PUT http://localhost:8000/api/users/h1@gmail.com
Content-Type: application/json
Accept-Language: ja

{
  "name": "updatedUser1",
  "email": "h4@gmail.com",
  "subscription_plan": "pro",
  "role": "developer"
}
