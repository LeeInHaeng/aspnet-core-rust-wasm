using Microsoft.AspNetCore.Mvc;

namespace HelloRustWasm.Controllers
{
    public class HelloWasmController : Controller
    {
        public IActionResult Index()
        {
            return View();
        }
    }
}
