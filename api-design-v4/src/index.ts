import config from './config';
import app from './server'
import * as dotenv from 'dotenv';

dotenv.config();
const HOST = "192.168.0.173";

app.listen(config.port, HOST, () => {
  console.log(`running on http://${HOST}:${config.port}`)
})
