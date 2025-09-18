'use client'

import React, { createContext, useContext, ReactNode } from 'react'
import { ThemeProvider as NextThemeProvider } from 'next-themes'

interface ThemeProviderProps {
  children: ReactNode
}

const ThemeContext = createContext<Record<string, unknown> | undefined>(undefined)

export function ThemeProvider({ children }: ThemeProviderProps) {
  return (
    <NextThemeProvider
      attribute="class"
      defaultTheme="dark"
      enableSystem
      disableTransitionOnChange
    >
      {children}
    </NextThemeProvider>
  )
}

export function useTheme() {
  const context = useContext(ThemeContext)
  return context
}
