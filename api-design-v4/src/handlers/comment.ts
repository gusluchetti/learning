import prisma from '../db'
import { Request, Response } from 'express';

export const createComment = async (req: Request, res: Response, next) => {
  try {
    const created = await prisma.comments.create({
      data: {
        post_id: req.body.pid,
        comment: req.body.comment,
      }
    })
    return res.json({ data: created });
  } catch (error) {
    next(error)
  }
}

export const getComments = async (_: Request, res: Response, next) => {
  try {
    const comments = await prisma.comments.findMany();
    return res.json({ data: comments });
  } catch (error) {
    next(error)
  }
}

export const getComment = async (req: Request, res: Response, next) => {
  try {
    let numId = parseInt(req.params.id);
    const comment = await prisma.comments.findUnique({
      where: { cid: numId }
    });
    return res.json({ data: comment });
  } catch (error) {
    next(error)
  }
}

export const updateComment = async (req: Request, res: Response, next) => {
  try {
    let numId = parseInt(req.params.id);
    const update = await prisma.comments.update({
      where: { cid: numId },
      data: {
        comment: req.params.comment || undefined,
      }
    })
    return res.json({ data: update });
  } catch (error) {
    next(error);
  }
}

export const deleteComment = async (req: Request, res: Response, next) => {
  try {
    let numId = parseInt(req.params.id);
    const comment = await prisma.comments.delete({
      where: { cid: numId }
    });
    return res.json({ data: comment });
  } catch (error) {
    next(error);
  }
}
