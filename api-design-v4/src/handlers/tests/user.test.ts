import * as user from '../user'

describe('user handler', () => {
  it('should create new user', async () => {
    const req = { body: { username: 'hello', password: 'hi' } }
    const res = {
      json({ token }) {
        expect(token).toBeTruthy()
      }
    }

    const newUser = await user.createUser(req, res)
  })
})
