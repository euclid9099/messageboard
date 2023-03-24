using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace WPFClient
{
    internal class Response<TContent>
    {
        string? message;
        ErrorResponse? error;
        TContent? content;

        public Response() { }

        public string Message
        {
            get { return message ?? string.Empty; }
            set { message = value; }
        }

        public ErrorResponse? Error
        {
            get { return error; }
            set { error = value; }
        }

        public TContent? Content
        {
            get { return content; }
            set { content = value; }
        }
    }
}
