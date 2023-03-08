import { Router, Request, Response } from 'https://deno.land/x/oak@v11.1.0/mod.ts';
import { login, signup } from './controllers/security.ts';
import { getUsers, getUser, editUser, addPOI, removePOI, deleteUser } from './controllers/user.ts';

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
router.get('/users', getUsers);             //get all users ordered by connections
router.get('/users/:id', getUser);          //get one user
router.patch('/users/:id', editUser);       //update one user data (only as self)
router.post('/users/:id/addPOI', addPOI);          //add a person of interest
router.post('/users/:id/removePOI', removePOI);          //add a person of interest
router.delete('/users/:id', deleteUser);    //delete a user (only as self or admin)


//posts

export default router