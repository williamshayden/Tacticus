import { test, expect } from '@playwright/test';

test.describe('Training Mode', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('.loading-text')).not.toBeVisible({ timeout: 10000 });
    
    // Complete onboarding if shown
    const welcomeVisible = await page.locator('text=Welcome to Tacticus').isVisible();
    if (welcomeVisible) {
      await page.locator('input[placeholder="Enter your name"]').fill('TestPlayer');
      await page.locator('button:has-text("Continue")').click();
      await page.locator('text=Beginner').click();
      await page.locator('button:has-text("Begin Training")').click();
    }
    
    // Navigate to training
    await page.locator('button:has-text("Continue Training")').click();
  });

  test('should show exercise with chess board', async ({ page }) => {
    await expect(page.locator('.chess-board-container')).toBeVisible();
  });

  test('should show exercise info', async ({ page }) => {
    await expect(page.locator('text=Exercise Info')).toBeVisible();
  });

  test('should show session progress', async ({ page }) => {
    await expect(page.locator('text=Session Progress')).toBeVisible();
    await expect(page.locator('text=Score')).toBeVisible();
    await expect(page.locator('text=Streak')).toBeVisible();
  });

  test('should have hint button', async ({ page }) => {
    await expect(page.locator('button:has-text("Hint")')).toBeVisible();
  });

  test('should be able to end session', async ({ page }) => {
    await page.locator('button:has-text("End Session")').click();
    // Should return to hub
    await expect(page.locator('text=TRAIN')).toBeVisible();
  });
});
