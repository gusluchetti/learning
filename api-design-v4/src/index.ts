import * as dotenv from 'dotenv';
import express from 'express';
import morgan from 'morgan';
import router from './router';

import { protect } from './modules/auth';
import { createUser, signIn } from './handlers/user';
import config from './config';

dotenv.config();
const HOST = "192.168.0.173";

const app = express()

app.use(morgan('dev'))
app.use(express.json())
app.use(express.urlencoded({ extended: true }))

app.use('/api', protect, router)

app.post('/user', createUser)
app.post('/signin', signIn)

app.use((err, req, res, next) => {
  switch (err.type) {
    case 'auth':
      res.status(401).json({ message: 'Unauthorized' })
      break;

    case 'input':
      res.status(400).json({ message: 'Invalid Input' })
      break;

    default:
      res.status(500).json({ message: 'Internal Server Error' })
      break;
  }
  console.log(err)
})

app.listen(config.port, HOST, () => {
  console.log(`running on http://${HOST}:${config.port}`)
})
