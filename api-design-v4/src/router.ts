import { Router } from 'express'
import { body } from 'express-validator';
import { handleInputErrors } from './modules/middleware';

import prisma from './db';

const router = Router()
// reminder: put is completely replace, patch is just change what was sent

// posts
router.get('/posts', () => { })

router.post('/post',
  body('title').notEmpty().isString(),
  body('body').notEmpty().isString(),
  handleInputErrors,
  (req, res) => {
    res.end()
  })

router.get('/post/:id', () => { })

router.patch('/post/:id',
  body('title').optional().isString(),
  body('body').optional().isString(),
  handleInputErrors,
  (req, res) => {
    res.end()
  })

router.delete('/post/:id', () => { })

// comments
router.get('/comments/:post_id', () => { })

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

router.delete('/comment:id', () => { })

export default router
