import { test, expect } from '@playwright/test';

test.describe('Play Mode', () => {
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
    
    // Navigate to play
    await page.locator('button:has-text("Play Now")').click();
  });

  test('should show game configuration', async ({ page }) => {
    await expect(page.locator('text=New Game')).toBeVisible();
    await expect(page.locator('text=Time Control')).toBeVisible();
    await expect(page.locator('text=Play As')).toBeVisible();
  });

  test('should show engine info', async ({ page }) => {
    await expect(page.locator('text=Tacticus Engine')).toBeVisible();
    await expect(page.locator('text=ELO')).toBeVisible();
  });

  test('should have color selection options', async ({ page }) => {
    await expect(page.locator('text=White')).toBeVisible();
    await expect(page.locator('text=Random')).toBeVisible();
    await expect(page.locator('text=Black')).toBeVisible();
  });

  test('should start game when clicking Start Game', async ({ page }) => {
    await page.locator('button:has-text("Start Game")').click();
    
    // Should show chess board and game status
    await expect(page.locator('.chess-board-container')).toBeVisible();
    await expect(page.locator('text=Game Status')).toBeVisible();
  });

  test('should be able to cancel and return to hub', async ({ page }) => {
    await page.locator('button:has-text("Cancel")').click();
    await expect(page.locator('text=TRAIN')).toBeVisible();
  });
});
