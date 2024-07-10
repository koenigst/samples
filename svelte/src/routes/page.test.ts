import { describe, it, expect } from 'vitest'
import { render } from '@testing-library/svelte'
import Page from './+page.svelte'

const data = { user: 'Asterix' }

describe('home page', () => {
	it('has title with name', async () => {
		const result = render(Page, { data })

		const heading = result.findByRole('heading')

		await expect(heading).resolves.toHaveTextContent('Hello Asterix!')
	})
})
