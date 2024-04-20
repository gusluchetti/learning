import { Router } from 'express'
import { body } from 'express-validator';
import { handleInputErrors } from './modules/middleware';

import prisma from './db';

const router = Router()
// reminder: put is completely replace, patch is just change what was sent

// posts
router.get('/posts', async (req, res) => {
  const posts = await prisma.posts.findMany();
  return res.json(posts);
})
router.get('/post/:id', async (req, res) => {
  let numId = parseInt(req.params.id);
  const post = await prisma.posts.findUnique({
    where: { pid: numId }
  });
  return res.json(post);
})

router.post('/post',
  body('title').notEmpty().isString(),
  body('body').notEmpty().isString(),
  handleInputErrors,
  async (req, res) => {
    const createPost = await prisma.posts.create({
      data: {
        title: req.body.title,
        body: req.body.body,
      }
    })

    return res.json(createPost);
  });

router.patch('/post/:id',
  body('title').optional().isString(),
  body('body').optional().isString(),
  handleInputErrors,
  (req, res) => {
    res.end()
  })

router.delete('/post/:id', async (req, res) => {
  let numId = parseInt(req.params.id);
  const post = await prisma.posts.delete({
    where: { pid: numId }
  });
  return res.json(post);
})

// comments
router.get('/comments/:post_id', async (req, res) => {
  let numId = parseInt(req.params.post_id);
  const comments = await prisma.comments.findMany({
    where: { post_id: numId }
  });
  return res.json(comments);
})

router.post('/comment',
  body('pid').notEmpty().isNumeric(),
  body('comment').notEmpty().isString(),
  handleInputErrors,
  (req, res) => {
    res.end()
  })

router.put('/comment/:id',
  body('comment').notEmpty().isString(),
  handleInputErrors,
  (req, res) => {
    res.end()
  })

router.delete('/comment:id', async (req, res) => {
  let numId = parseInt(req.params.id);
  const comment = await prisma.comments.delete({
    where: { cid: numId }
  });
  return res.json(comment);
})

export default router
