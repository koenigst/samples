// @vitest-environment jsdom

import { describe, it, expect } from 'vitest'
import { render } from '@testing-library/svelte'
import Page from './+page.svelte'
import type { LinkData } from './+layout.server'
import type { Users } from '$lib'

const myself = { id: 1, first_name: 'Asterix' }
const firstFriend = { id: 42, first_name: 'Obelix' }

const users: Promise<Users> = Promise.resolve({ myself, friends: [firstFriend] })
const links: LinkData[] = []
const data = { users, links }

describe('home page', () => {
	it('has title with name', async () => {
		const result = render(Page, { data })

		const heading = result.findByRole('heading')

		await expect(heading).resolves.toHaveTextContent('Hello Asterix!')
	})

	it('has link to friend', async () => {
		const result = render(Page, { data })

		const link = result.findByRole('link')

		await expect(link).resolves.toHaveTextContent('Obelix')
		await expect(link).resolves.toHaveAttribute('href', '/chats/42')
	})
})
