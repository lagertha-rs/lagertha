// @test feature = "bootstrap.main-method"
// @test description = "Verifies startup invokes a conventional public static main method and completes successfully."
// @test category = "success"

package bootstrap.main;

public class HelloWorldTest {
    public static void main(String[] args) {
        System.out.println("Hello World!");
    }
}
