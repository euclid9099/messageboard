import { Router, Request, Response } from 'https://deno.land/x/oak/mod.ts';
import { getBooks, getBook, addBook, updateBook, deleteBook } from './controllers/books.ts';
import { login, signup } from './controllers/security.ts';

const router = new Router()

router.get('/', ({ request, response }: { request: Request; response: Response }) => {
    response.status = 200;
    response.body = {
        login: `${request.url.origin}/login`,
        signup: `${request.url.origin}/signup`,
        users: `${request.url.origin}/users`,
        posts: `${request.url.origin}/posts`,
    };
});

//authentication
router.post('/login', login);   //login with username and password
router.post('/signup', signup); //sign up with username and password

//user

//posts


router.get('/books', getBooks)
      .get('/books/:isbn', getBook)
      .post('/books', addBook)
      .put('/books/:isbn', updateBook)
      .delete('/books/:isbn', deleteBook)
      /**/

export default router