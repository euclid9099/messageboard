using System;
using System.Collections.Generic;
using System.Linq;
using System.Security.Principal;
using System.Text;
using System.Threading.Tasks;
using System.Windows;

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

    internal class PostResponse
    {
        string? author;
        bool? edited;
        string? id;
        string? message;
        DateTime time;
        int likes;
        int dislikes;

        public PostResponse() { }

        public string Author
        {
            get { return author ?? "(unknown)"; }
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
            get { return $"Like ({likes})"; }
        }

        public string DisplayDislikes
        {
            get { return $"Dislike ({dislikes})"; }
        }
    }
}
