package exceptions.handling.catch_subclass;

import java.io.FileNotFoundException;
import java.io.IOException;

public class CatchSubclassOkMain {
    public static void main(String[] args) {
        try {
            throw new FileNotFoundException("file not found");
        } catch (IOException e) {
            System.out.println("Caught IOException (superclass)");
        }
    }
}