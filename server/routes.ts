import { Router, Request, Response } from 'https://deno.land/x/oak@v11.1.0/mod.ts';
import { login, signup } from './controllers/security.ts';
import { getUsers, getUser, editUser, addPOI, removePOI } from './controllers/user.ts';
import { createPost, getPosts, getPost, editPost, deletePost } from "./controllers/post.ts";

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
//router.delete('/users/:id', deleteUser);    //delete doesn't work from deno so I procrastinate this point


//posts
router.post('/posts', createPost);      //post something
router.get('/posts', getPosts);         //get all posts by interactions
router.get('/posts/:id', getPost);      //get one specific post
router.patch('/posts/:id', editPost);   //edit a post (only if authorized)
//router.post('/posts/:id/like', likePost);       //add self to people who like - mutually exclusive with dislike list
//router.post('/posts/:id/dislike', dislikePost);    //add self to people who dislike - mutually exclusive with like list
//router.post('/posts/:id/respond', createResponse);      //respond to a post
//router.get('/posts/:id/responses', getResponses);        //get responses to a post
router.delete('/posts/:id', deletePost);        //delete a post - moves all child posts upp


export default router