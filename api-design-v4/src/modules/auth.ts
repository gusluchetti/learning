import jwt from 'jsonwebtoken';
import * as bcrypt from 'bcrypt';

export const comparePasswords = (password: string, hash: string) => {
  return bcrypt.compare(password, hash);
}

export const hashPassword = (password: string) => {
  return bcrypt.hash(password, 80)
}

export const createJWT = (id: string, username: string) => {
  const token = jwt.sign(
    { id: id, username: username },
    process.env.JWT_SECRET
  )
  return token;
}

export const protect = (req, res, next) => {
  const bearer = req.headers.authorization;
  if (!bearer) {
    res.status(401).send('Not Authorized: Missing Bearer');
    return;
  }

  const [, token] = bearer.split(" ")
  if (!token) {
    res.status(401).send('Not Authorized: Missing Token');
    return;
  }

  try {
    const user = jwt.verify(token, process.env.JWT_SECRET)
    req.user = user
    next();
  } catch (e) {
    console.error(e);
    res.status(401).send('Not Authorized: Invalid Token');
    return;
  }
}
