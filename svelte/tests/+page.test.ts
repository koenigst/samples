import { expect, test } from '@playwright/test'

test('home page has expected title', async ({ page }) => {
	await page.goto('/')
	await expect(page.getByRole('heading')).toHaveText('Hello George!')
})

test('home page has friend', async ({ page }) => {
	await page.goto('/')
	await expect(page.getByRole('listitem').first()).toHaveText('Janet')
})
