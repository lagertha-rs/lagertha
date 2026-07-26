// @test feature = "execution.fields.access"
// @test description = "Verifies an instance field read through a null receiver throws NullPointerException."
// @test category = "error"

package execution.objects.fields;

public class NullFieldAccessTest {
    public static void main(String[] args) {
        NullFieldReceiver receiver = null;
        System.out.println(receiver.value);
    }
}

class NullFieldReceiver {
    int value;
}
