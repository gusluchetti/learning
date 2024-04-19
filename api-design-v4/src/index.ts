import express from 'express';
import morgan from 'morgan';
import router from './router';

const PORT = 3001;
const HOST = "192.168.0.173";

const app = express()

app.use(morgan('dev'))

app.get('/', (req, res) => {
  console.log(req, 'hello from express!')
  res.status(200)
  res.json({ message: 'hi there!' })
})

app.use('/api', router)

app.listen(PORT, HOST, () => {
  console.log(`running on http://${HOST}:${PORT}`)
})
