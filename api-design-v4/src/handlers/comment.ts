import prisma from '../db'
import { Request, Response } from 'express';

export const createComment = async (req: Request, res: Response) => {
  const created = await prisma.comments.create({
    data: {
      post_id: req.body.pid,
      comment: req.body.comment,
    }
  })
  return res.json({ data: created });
}

export const getComments = async (_: Request, res: Response) => {
  const comments = await prisma.comments.findMany();
  return res.json({ data: comments });
}

export const getComment = async (req: Request, res: Response) => {
  let numId = parseInt(req.params.id);
  const comment = await prisma.comments.findUnique({
    where: { cid: numId }
  });
  return res.json({ data: comment });
}

export const updateComment = async (req: Request, res: Response) => {
  let numId = parseInt(req.params.id);
  const update = await prisma.comments.update({
    where: { cid: numId },
    data: {
      comment: req.params.comment || undefined,
    }
  })

  return res.json({ data: update });
}

export const deleteComment = async (req: Request, res: Response) => {
  let numId = parseInt(req.params.id);
  const comment = await prisma.comments.delete({
    where: { cid: numId }
  });
  return res.json({ data: comment });
}
