import jwt from 'jsonwebtoken';

export const createJWT = (user) => {
  const token = jwt.sign(
    { id: user.id, username: user.username },
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
