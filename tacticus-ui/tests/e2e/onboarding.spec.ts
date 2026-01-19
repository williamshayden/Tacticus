import { test, expect } from '@playwright/test';

test.describe('Onboarding Flow', () => {
  test('should show welcome screen on first launch', async ({ page }) => {
    await page.goto('/');
    
    // Wait for loading to complete
    await expect(page.locator('.loading-text')).not.toBeVisible({ timeout: 10000 });
    
    // Should show onboarding if first time
    await expect(page.locator('text=Welcome to Tacticus')).toBeVisible();
    await expect(page.locator('text=Gurgeh')).toBeVisible();
  });

  test('should allow entering name and continue', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('.loading-text')).not.toBeVisible({ timeout: 10000 });
    
    // Enter name
    const nameInput = page.locator('input[placeholder="Enter your name"]');
    await nameInput.fill('TestPlayer');
    
    // Continue button should be enabled
    const continueButton = page.locator('button:has-text("Continue")');
    await expect(continueButton).toBeEnabled();
    
    // Click continue
    await continueButton.click();
    
    // Should show skill selection
    await expect(page.locator('text=current chess experience')).toBeVisible();
  });

  test('should allow selecting skill level', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('.loading-text')).not.toBeVisible({ timeout: 10000 });
    
    // Enter name and continue
    await page.locator('input[placeholder="Enter your name"]').fill('TestPlayer');
    await page.locator('button:has-text("Continue")').click();
    
    // Select beginner
    await page.locator('text=Beginner').click();
    
    // Begin Training button should be enabled
    await expect(page.locator('button:has-text("Begin Training")')).toBeEnabled();
  });
});
