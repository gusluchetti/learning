import { Router } from 'express'
import { body, validationResult } from 'express-validator';
import prisma from './db';

const router = Router()
// reminder: put is completely replace, patch is just change what was sent

// posts
router.get('/posts', () => { })
router.post('/post', () => { })
router.get('/post/:id', () => { })
router.put('/post/:id', () => { })
router.delete('/post/:id', () => { })

// comments
router.get('/comments/:post_id', () => { })
router.post('/comment', () => { })
router.put('/comment/:id', body('comment'), (req, res) => {
  const errors = validationResult(req);
  if (!errors.isEmpty()) {
    res.status(400);
    res.json({ errors: errors.array() })
  }
  res.end()
})
router.delete('/comment:id', () => { })

export default router
