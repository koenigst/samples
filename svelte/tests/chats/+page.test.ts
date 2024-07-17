import { expect, test } from '@playwright/test'

test('chat page has expected title', async ({ page }) => {
	await page.goto('/')
	await page.getByText('Emma').click()
	await expect(page.getByRole('heading')).toHaveText('Chat with Emma')
})

test('chat page can send message', async ({ page }) => {
	await page.goto('/')
	await page.getByText('Emma').click()
	await page.getByRole('textbox').fill('my test message')
	await page.getByText('send').click()
	await expect(page.getByRole('paragraph').last()).toHaveText('my test message')
	await expect(page.getByRole('textbox')).toBeEmpty()
})
