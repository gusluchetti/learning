const express = require('express')

const app = express()

app.get('/', (req, res) => {
  console.log(req, 'hello from express!')
  res.status(200)
  res.json({ message: 'hi there!' })
})

app.listen(3001, "192.168.0.173")
