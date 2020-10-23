import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequestAndVerify(findTestObject('Leave/GET_Employee_Leave_Requests', [('url') : GlobalVariable.url, ('token') : GlobalVariable.token]))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'data[0].employeeName', "Zahra test test")
WS.verifyElementPropertyValue(response, 'data[0].employeeId', "2")
WS.verifyElementPropertyValue(response, 'data[0].id', "18")
WS.verifyElementPropertyValue(response, 'data[0].fromDate', "2020-10-27")
WS.verifyElementPropertyValue(response, 'data[0].toDate', "2020-10-28")
WS.verifyElementPropertyValue(response, 'data[0].type', "Cuti")
WS.verifyElementPropertyValue(response, 'data[0].leaveBalance', "-5.00")
WS.verifyElementPropertyValue(response, 'data[0].numberOfDays', "2.00")
WS.verifyElementPropertyValue(response, 'data[0].days[0].date', "2020-10-28")
WS.verifyElementPropertyValue(response, 'data[0].days[0].status', "SCHEDULED")
WS.verifyElementPropertyValue(response, 'data[0].days[0].duration', "8.00")
WS.verifyElementPropertyValue(response, 'data[0].days[1].date', "2020-10-27")
WS.verifyElementPropertyValue(response, 'data[0].days[1].status', "SCHEDULED")
WS.verifyElementPropertyValue(response, 'data[0].days[1].duration', "8.00")

WS.verifyElementPropertyValue(response, 'data[2].employeeName', "Zahra test test")
WS.verifyElementPropertyValue(response, 'data[2].employeeId', "2")
WS.verifyElementPropertyValue(response, 'data[2].id', "1")
WS.verifyElementPropertyValue(response, 'data[2].fromDate', "2020-10-20")
WS.verifyElementPropertyValue(response, 'data[2].toDate', "2020-10-20")
WS.verifyElementPropertyValue(response, 'data[2].type', "Cuti")
WS.verifyElementPropertyValue(response, 'data[2].leaveBalance', "-5.00")
WS.verifyElementPropertyValue(response, 'data[2].numberOfDays', "1.00")
WS.verifyElementPropertyValue(response, 'data[2].days[0].date', "2020-10-20")
WS.verifyElementPropertyValue(response, 'data[2].days[0].status', "TAKEN")
WS.verifyElementPropertyValue(response, 'data[2].days[0].duration', "8.00")