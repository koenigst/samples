import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import { ReactNode, Suspense } from 'react'
import './globals.css'
import { loadChatNavigation } from './chat-navigation'
import Link from 'next/link'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
	title: 'koenigst NEXT.js sample app',
}

type LayoutProps = { children: ReactNode[], }

async function ChatsNavigation() {
	const chats = await loadChatNavigation()

	return <>
		{chats.map(chat => <Link key={chat.href} href={chat.href}>{chat.name}</Link>)}
	</>
}

async function Navigation() {
	return <nav>
		<Link href="/">friends</Link>
		<Suspense fallback={<span>...</span>}>
			<ChatsNavigation />
		</Suspense>
	</nav>
}

export default function RootLayout({children,}: LayoutProps) {
	return (
		<html lang="en">
			<body className={inter.className}>
				<Navigation />
				{children}
			</body>
		</html>
	)
}
