import { test, expect } from "@playwright/test";

test.describe("Auth Flow", () => {
  test("should register a new user", async ({ page }) => {
    await page.goto("http://localhost:3000/auth/register");

    await page.fill("#name", "John Doe");
    await page.fill("#email", "hardleberg@gmail.com");
    await page.fill("#password", "Securepassword123.");
    await page.fill("#password_confirmation", "Securepassword123.");

    await page.click('button[type="submit"]');

    await page.waitForURL("**/auth/login", { timeout: 100000 });
  });

  test("should login an existing user", async ({ page }) => {
    await page.goto("http://localhost:3000/auth/login");

    await page.fill("#email", "hardleberg@gmail.com");
    await page.fill("#password", "Securepassword123.");

    await page.click('button[type="submit"]');

    await expect(page).toHaveURL("http://localhost:3000/");
  });
});
