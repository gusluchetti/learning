import prisma from "../db";
import { comparePasswords, createJWT, hashPassword } from "../modules/auth";
import { Request, Response } from 'express';

export const createUser = async (req: any, res: Response) => {
  const hashed = await hashPassword(req.body.password);
  if (!hashed) {
    res.status(500).send('whoops!')
    return;
  }

  const user = await prisma.users.create({
    data: {
      username: req.body.username,
      password: hashed
    }
  })

  const token = createJWT(user.uid, user.username);
  res.json({ token });
}

export const signIn = async (req: Request, res: Response) => {
  const user = await prisma.users.findUnique({
    where: { username: req.body.username }
  })

  const isValidUser = await comparePasswords(req.body.password, user.password);
  if (!isValidUser) {
    res.status(401).send('Wrong username/password')
    return;
  }

  const token = createJWT(user.uid, user.username);
  res.json({ token });
}
