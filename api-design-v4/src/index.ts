import express from 'express';
import router from './router'

const app = express()
const PORT = 3001
const HOST = "192.168.0.173"

app.get('/', (req, res) => {
  console.log(req, 'hello from express!')
  res.status(200)
  res.json({ message: 'hi there!' })
})

app.use('/api', router)

app.listen(PORT, HOST, () => {
  console.log('running on http://192.168.0.173:3001')
})
