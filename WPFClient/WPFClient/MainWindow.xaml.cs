using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Net.Http;
using System.Net.Http.Json;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace WPFClient
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        private string api = "http://localhost:7700";

        private string? token;
        private bool accountActive;
        private DateTime mostRecentPost;
        private HttpClient httpClient;

        public MainWindow()
        {
            InitializeComponent();
            httpClient = new HttpClient();
            accountActive = false;
            mostRecentPost = DateTime.MinValue;
        }

        private void ActivateAccount(Object sender, RoutedEventArgs e)
        {
            accountActive = true;
            accountDisplay.Fill = new SolidColorBrush(Colors.LimeGreen);
        }

        private void DeactivateAccount(Object sender, RoutedEventArgs e)
        {
            accountActive = false;
            accountDisplay.Fill = new SolidColorBrush(Colors.Gray);
        }

        private void Login(object sender, RoutedEventArgs e)
        {
            string uname = usernameTextbox.Text;
            if (string.IsNullOrEmpty(uname)) return;

            string body = $"{{\"username\": \"{uname}\",\"password\": \"{passwordTextbox.Password}\"}}";
            var response = httpClient.PostAsync($"{api}/login", new StringContent(body, Encoding.UTF8, "application/json")).Result;
            var resBody = response.Content.ReadFromJsonAsync<Response<AuthorizeResponse>>().Result;

            if (resBody?.Content == null)
            {
                MessageBox.Show(resBody?.Error?.Message, resBody?.Error?.Name, MessageBoxButton.OK, MessageBoxImage.Error);
                return;
            }

            this.token = resBody.Content.Token;

            accountTab.Text = $"Account ({uname})";

            useAccountCheckbox.IsChecked = true;

            loginView.Visibility = Visibility.Hidden;
            accountView.Visibility = Visibility.Visible;
        }

        private void Logout(object sender, RoutedEventArgs e)
        {
            DeactivateAccount(sender, e);

            accountTab.Text = "Account (anonymus)";

            accountView.Visibility = Visibility.Hidden;
            loginView.Visibility = Visibility.Visible;
        }

        private void LoadMorePostsButton(Object sender, RoutedEventArgs e)
        {
            var response = httpClient.GetFromJsonAsync<Response<DBResponse<List<PostResponse>>>>($"{api}/posts?after={mostRecentPost.ToString("o", CultureInfo.InvariantCulture)}").Result;

            if (response?.Content == null || response.Content.Result.Count == 0) return;
            
            foreach (var post in response.Content.Result)
            {
                postsListbox.Items.Add(post);
            }
            mostRecentPost = response.Content.Result.Last().Time;
        }
    }
}
