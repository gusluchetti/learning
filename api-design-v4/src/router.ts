import { Router } from 'express'

const router = Router()

// posts
router.get('/posts', (req, res) => {
  res.json(['post1', 'post2'])
  res.end()
})
router.post('/post', () => { })
router.get('/post:id', () => { })
router.put('/post:id', () => { }) // COMPLETELY REPLACE
router.delete('/post:id', () => { })

// comments
router.get('/comments:post_id', () => { })
router.post('/comment', () => { })
router.put('/comment:id', () => { })
router.delete('/comment:id', () => { })

export default router
