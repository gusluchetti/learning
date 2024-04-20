import * as dotenv from 'dotenv';
import express from 'express';
import morgan from 'morgan';
import router from './router';

import { protect } from './modules/auth';
import { createUser, signIn } from './handlers/user';

dotenv.config();
const PORT = 3001;
const HOST = "192.168.0.173";

const app = express()

app.use(morgan('dev'))
app.use(express.json())
app.use(express.urlencoded({ extended: true }))

app.use('/api', protect, router)

app.post('/user', createUser)
app.post('/signin', signIn)

app.use((err, req, res, next) => {
  console.error(err)
  res.json({ message: `error: ${err.message}` })
})

app.listen(PORT, HOST, () => {
  console.log(`running on http://${HOST}:${PORT}`)
})
