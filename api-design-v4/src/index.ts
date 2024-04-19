import express from 'express';
import morgan from 'morgan';
import router from './router';

import * as dotenv from 'dotenv';
import { protect } from './modules/auth';
dotenv.config();

const PORT = 3001;
const HOST = "192.168.0.173";

const app = express()

app.use(morgan('dev'))
app.use(express.json())
app.use(express.urlencoded({ extended: true }))

app.get('/', (_, res) => {
  res.json({ message: 'hi there!' })
})

app.use('/api', protect, router)

app.listen(PORT, HOST, () => {
  console.log(`running on http://${HOST}:${PORT}`)
})
