import { test, expect } from '@playwright/test';

test.describe('Main Hub', () => {
  // Skip onboarding by creating profile first
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
  });

  test('should show hub with all sections', async ({ page }) => {
    // Should show the four main cards
    await expect(page.locator('text=TRAIN')).toBeVisible();
    await expect(page.locator('text=PLAY')).toBeVisible();
    await expect(page.locator('text=ANALYZE')).toBeVisible();
    await expect(page.locator('text=LEARN')).toBeVisible();
  });

  test('should show user stats', async ({ page }) => {
    await expect(page.locator('text=Rating')).toBeVisible();
    await expect(page.locator('text=Streak')).toBeVisible();
  });

  test('should show Gurgeh message', async ({ page }) => {
    await expect(page.locator('text=Gurgeh:')).toBeVisible();
  });

  test('should navigate to training', async ({ page }) => {
    await page.locator('button:has-text("Continue Training")').click();
    await expect(page.locator('text=Exercise')).toBeVisible();
  });

  test('should navigate to play', async ({ page }) => {
    await page.locator('button:has-text("Play Now")').click();
    await expect(page.locator('text=New Game')).toBeVisible();
  });
});
