import { Hono } from 'hono'
import { health } from './routes/health'

export const app = new Hono()

app.route('/', health)
