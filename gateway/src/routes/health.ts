import { Hono } from 'hono'

export const health = new Hono()

health.get('/health', (c) => {
  return c.json({ status: 'ok' })
})
