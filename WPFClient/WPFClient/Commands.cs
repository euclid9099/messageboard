using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Input;
using System.Xml.Linq;

namespace WPFClient
{
    internal class LikePostCommand: ICommand
    {
        public event EventHandler? CanExecuteChanged;

        public bool CanExecute(object? parameter)
        {
            return (parameter as PostResponse)?.Liked == null ? false : true;
        }

        public void Execute(object? parameter)
        {
            Debug.WriteLine((parameter as PostResponse)?.Id);
        }

    }
}
