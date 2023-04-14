using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Security.Principal;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Input;

namespace WPFClient
{
    internal class ErrorResponse
    {
        string? name;
        string? message;

        public ErrorResponse() { }

        public string Name
        {
            get { return name ?? string.Empty; }
            set { name = value; }
        }

        public string Message
        {
            get { return message ?? string.Empty; }
            set { message = value; }
        }
    }

    internal class AuthorizeResponse
    {
        string? token;

        public AuthorizeResponse() { }

        public string Token
        {
            get { return token ?? string.Empty; }
            set { token = value; }
        }
    }

    internal class DBResponse<TSubResponse>
    {
        string status;
        TSubResponse result;

        public DBResponse() { }

        public string Status
        {
            get { return status; }
            set { status = value; }
        }

        public TSubResponse Result
        {
            get { return result; }
            set { result = value; }
        }
    }

    internal class User
    {
        public User() { }
        public string Username { get; set; }
        public string Id { get; set; }
        public bool Admin { get; set; }
    }

    class PostResponse
    {
        User? author;
        bool? edited;
        string? id;
        string? message;
        DateTime time;
        int likes;
        bool? liked;
        int dislikes;
        bool? disliked;

        public PostResponse() { }

        public string AuthorName
        {
            get { return author?.Username ?? "(unknown)"; }
        }

        public User? Author {
            get { return author; }
            set { author = value; }
        }

        public bool Edited
        {
            get { return edited ?? false; }
            set { edited = value; }
        }

        public string Id
        {
            get { return id ?? string.Empty; }
            set { id = value; }
        }

        public string Message
        {
            get { return message ?? string.Empty; }
            set { message = value; }
        }

        public DateTime Time
        {
            get { return time; }
            set { time = value; }
        }

        public int Likes
        {
            get { return likes; }
            set { likes = value; }
        }

        public int Dislikes
        {
            get { return dislikes; }
            set { dislikes = value; }
        }

        public bool? Liked
        {
            get { return liked; }
            set { liked = value; }
        }

        public bool? Disliked
        {
            get { return disliked; }
            set { disliked = value; }
        }

        public string DisplayDate
        {
            get { return time.ToString(); }
        }

        public string DisplayEdited
        {
            get { return (edited??false) ? "(edited)" : ""; }
        }

        public string DisplayLikes
        {
            get { return $"  Like ({likes})  "; }
        }

        public string DisplayDislikes
        {
            get { return $"  Dislike ({dislikes})  "; }
        }
    }
}
