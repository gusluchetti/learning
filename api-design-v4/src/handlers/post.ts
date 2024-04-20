import prisma from '../db'
import { Request, Response } from 'express';

export const createPost = async (req: Request, res: Response) => {
  const created = await prisma.posts.create({
    data: {
      title: req.body.title,
      body: req.body.body,
    }
  })
  return res.json({ data: created });
}

export const getPosts = async (_: Request, res: Response) => {
  const posts = await prisma.posts.findMany();
  return res.json({ data: posts });
}

export const getPost = async (req: Request, res: Response) => {
  let numId = parseInt(req.params.id);
  const post = await prisma.posts.findUnique({
    where: { pid: numId }
  });
  return res.json({ data: post });
}

export const updatePost = async (req: Request, res: Response) => {
  let numId = parseInt(req.params.id);
  const update = await prisma.posts.update({
    where: { pid: numId },
    data: {
      title: req.params.title || undefined,
      body: req.params.body || undefined,
    }
  })

  return res.json({ data: update });
}

export const deletePost = async (req: Request, res: Response) => {
  let numId = parseInt(req.params.id);
  const post = await prisma.posts.delete({
    where: { pid: numId }
  });
  return res.json({ data: post });
}
