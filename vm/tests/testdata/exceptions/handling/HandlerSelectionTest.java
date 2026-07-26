// @test feature = "exceptions.handler-selection"
// @test description = "Verifies exact, superclass, ordered, catch-all, nested, mismatched, and normal exception-table paths."
// @test category = "success"

package exceptions.handling;

public class HandlerSelectionTest {
    public static void main(String[] args) {
        SelectionChild child = new SelectionChild();
        int marker = 0;

        try {
            throw child;
        } catch (SelectionOther wrong) {
            marker = -1;
        } catch (SelectionChild exact) {
            assert exact == child : "handler.exact.identity";
            marker = 1;
        } catch (SelectionParent later) {
            marker = -2;
        }
        assert marker == 1 : "handler.first.match";

        try {
            throw child;
        } catch (SelectionParent parent) {
            assert parent == child : "handler.superclass.identity";
            marker = 2;
        }
        assert marker == 2 : "handler.superclass";

        try {
            throw new SelectionOther();
        } catch (Throwable caught) {
            assert caught instanceof SelectionOther : "handler.catch.all.type";
            marker = 3;
        }
        assert marker == 3 : "handler.catch.all";

        try {
            try {
                throw child;
            } catch (SelectionOther wrong) {
                marker = -3;
            }
        } catch (SelectionParent outer) {
            assert outer == child : "handler.outer.identity";
            marker = 4;
        }
        assert marker == 4 : "handler.mismatch.skipped";

        try {
            try {
                throw new SelectionOther();
            } catch (SelectionOther inner) {
                marker = 5;
            }
        } catch (Throwable outer) {
            marker = -4;
        }
        assert marker == 5 : "handler.nested.inner";

        try {
            marker = 6;
        } catch (Throwable unexpected) {
            marker = -5;
        }
        assert marker == 6 : "handler.normal.bypass";
    }
}

class SelectionParent extends RuntimeException {}
class SelectionChild extends SelectionParent {}
class SelectionOther extends RuntimeException {}
