import { Router, Request, Response } from "https://deno.land/x/oak@v11.1.0/mod.ts";
import { login, signup } from "./controllers/security.ts";
import {
	getUsers,
	getUser,
	editUser,
	followUser,
	unfollowUser,
	getUserFollowers,
	getUserFollowing,
	deleteUser,
} from "./controllers/user.ts";
import {
	createPost,
	getPosts,
	editPost,
	deletePost,
	likePost,
	dislikePost,
} from "./controllers/post.ts";

const router = new Router();

router.get("/", ({ request, response }: { request: Request; response: Response }) => {
	response.status = 200;
	response.body = {
		status: "ok",
		links: {
			login: `${request.url.origin}/login`,
			signup: `${request.url.origin}/signup`,
			users: `${request.url.origin}/users`,
			posts: `${request.url.origin}/posts`,
		},
	};
});

//authentication
router.post("/login", login); //login with username and password
router.post("/signup", signup); //sign up with username and password

//user
router.get("/users", getUsers); //get all users ordered by connections
router.get("/users/:id", getUser); //get one user
router.patch("/users/:id", editUser); //update one user data (only as self)
router.delete("/users/:id", deleteUser); //update one user data (only as self)
router.get("/users/:id/followers", getUserFollowers); //get followers
router.get("/users/:id/following", getUserFollowing); //get following
router.post("/users/:id/follow", followUser); //add a person of interest
router.post("/users/:id/unfollow", unfollowUser); //add a person of interest

//posts
router.post("/posts", createPost); //post something, use url parameter with ?parent=<id> to post as response
router.get("/posts", getPosts); //get all top level posts, user ?parent=<id> to get responses to a post
router.get("/posts/:id", getPosts); //get one specific post
router.patch("/posts/:id", editPost); //edit a post (only if authorized)
router.delete("/posts/:id", deletePost); //delete a post - moves all child posts upp
router.post("/posts/:id/like", likePost); //add self to people who like - mutually exclusive with dislike list
router.post("/posts/:id/dislike", dislikePost); //add self to people who dislike - mutually exclusive with like list

export default router;
