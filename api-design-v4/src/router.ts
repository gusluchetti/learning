import { Router } from 'express'
import { body } from 'express-validator';
import { handleInputErrors } from './modules/middleware';

import { createPost, deletePost, getPost, getPosts, updatePost } from './handlers/post';
import { createComment, deleteComment, getComment, updateComment } from './handlers/comment';

const router = Router()
// reminder: put is completely replace, patch is just change what was sent

// posts
router.get('/post/:id', getPost)
router.get('/posts', getPosts)

router.post('/post',
  body('title').notEmpty().isString(),
  body('body').notEmpty().isString(),
  handleInputErrors, createPost);

router.patch('/post/:id',
  body('title').optional().isString(),
  body('body').optional().isString(),
  handleInputErrors, updatePost)

router.delete('/post/:id', deletePost);

// comments
router.get('/comments/:post_id', getComment)

router.post('/comment',
  body('pid').notEmpty().isNumeric(),
  body('comment').notEmpty().isString(),
  handleInputErrors,
  createComment
)

router.put('/comment/:id',
  body('comment').notEmpty().isString(),
  handleInputErrors,
  updateComment
)

router.delete('/comment:id', deleteComment)

export default router
